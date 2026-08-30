#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State, LUA_TSTRING};
use gmod::{gmod13_close, gmod13_open, lua_function, lua_string};
use std::collections::HashMap;
use std::env;
use std::sync::LazyLock;

use debug_print::debug_println;

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

static MOD_NAME: &str = "environ";
#[cfg(feature = "gmcl")]
static REALM: &str = "cl";
#[cfg(not(feature = "gmcl"))]
static REALM: &str = "sv";

macro_rules! err {
    ($arg:literal) => {
        format!("{}: {}", MOD_NAME, $arg)
    };
}

#[cfg(not(windows))]
const PATH_SEP: &str = ":";
#[cfg(windows)]
const PATH_SEP: &str = ";";

type RustLuaFunction = unsafe extern "C-unwind" fn(State) -> i32;

/// Names resolved as module functions rather than as environment variables.
/// A variable that happens to share one of these names is shadowed by it --
/// reach for `environ.get_csv` and friends knowing that, or read the raw
/// value out of the process environment some other way.
static FUNC_MAP: LazyLock<HashMap<&'static str, RustLuaFunction>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    macro_rules! export {
        ($name:ident) => {
            m.insert(stringify!($name), $name as RustLuaFunction);
        };
        ($func:ident, $name:literal) => {
            m.insert($name, $func as RustLuaFunction);
        };
    }

    export!(get_path);
    export!(get_csv);
    export!(get_version);
    export!(get_build_info);

    m
});

/// Get the requested string index, agnostic of method call type.
///
/// If we're called as a colon method:
/// `environ:whatever("env_key")` -> `environ.whatever(environ, "env_key")` -> `(userdata, string)`
///
/// Whereas, if we're called as a dot method:
/// `environ.whatever("env_key")` -> `string`
///
/// I don't care which is done, so we support both.
/// However, documentation will only show dot methoding.
macro_rules! requested_index {
    ( $lua:ident ) => {{
        // Keyed off the raw type tag rather than State::get_type. That
        // returns lua_typename's string, which is unreliable here on two
        // counts: its spelling for a userdata differs between stock LuaJIT
        // ("userdata") and GMod's build, and calling it in GMod leaves a
        // value behind on the stack (Facepunch/garrysmod-issues#5134) --
        // which is why gmod-rs's own lua_type_name carries a workaround
        // that get_type does not.
        //
        // A string in slot 1 is a dot call. Anything else -- the userdata a
        // colon call passes, or the self argument __index is handed -- puts
        // the key in slot 2.
        if $lua.lua_type(1) == LUA_TSTRING {
            debug_println!("fetched as a dot method");
            $lua.check_string(1)
        } else {
            debug_println!("fetched as a colon method");
            $lua.check_string(2)
        }
    }};
}

#[lua_function]
unsafe fn index(lua: State) -> i32 {
    let str_idx = requested_index!(lua);
    debug_println!("__index({})", str_idx);

    let rtn: i32 = match FUNC_MAP.get(&*str_idx) {
        Some(func) => {
            lua.push_function(*func);
            1
        }
        None => {
            let env_var = env::var(str_idx.as_ref());
            match env_var {
                Ok(val) => {
                    debug_println!("{} -> {}: {}", MOD_NAME, str_idx, val);
                    lua.push_string(val.as_str())
                }
                Err(_err) => {
                    debug_println!("{} -> {} failed: {}", MOD_NAME, str_idx, _err);
                    lua.push_nil();
                }
            }

            1
        }
    };

    rtn
}

unsafe fn push_table(lua: State, split: Vec<&str>) {
    lua.create_table(split.len() as i32, 0);
    let mut i = 0;
    for s in split {
        let s = s.trim();
        if !s.is_empty() {
            i += 1;
            lua.push_string(s);
            lua.raw_seti(-2, i);
        }
    }
}

#[lua_function]
unsafe fn get_path(lua: State) -> i32 {
    let env_var = env::var("PATH");
    match env_var {
        Ok(val) => {
            let val = val.as_str();
            let split = val.split(PATH_SEP).collect::<Vec<&str>>();
            push_table(lua, split);
        }
        Err(_err) => {
            debug_println!("{} -> {}: {}", MOD_NAME, "PATH", _err);
            lua.new_table();
        }
    }

    1
}

#[lua_function]
unsafe fn get_csv(lua: State) -> i32 {
    let str_idx = requested_index!(lua);
    let env_var = env::var(str_idx.as_ref());
    match env_var {
        Ok(val) => {
            debug_println!("{} -> {}: {}", MOD_NAME, str_idx, val);
            let val = val.as_str();
            let split = val.split(",").collect::<Vec<&str>>();
            push_table(lua, split);
        }
        Err(_err) => {
            debug_println!("{} -> {} failed: {}", MOD_NAME, str_idx, _err);
            lua.new_table();
        }
    }

    1
}

#[lua_function]
unsafe fn get_version(lua: State) -> i32 {
    lua.push_string(build_info::PKG_VERSION);
    1
}

#[lua_function]
unsafe fn get_build_info(lua: State) -> i32 {
    macro_rules! set_field {
        ($key:literal, $push:expr) => {
            $push;
            lua.set_field(-2, lua_string!($key));
        };
    }
    macro_rules! set_opt_str_field {
        ($key:literal, $value:expr) => {
            match $value {
                Some(v) => lua.push_string(v),
                None => lua.push_nil(),
            }
            lua.set_field(-2, lua_string!($key));
        };
    }

    lua.create_table(0, 11);
    set_field!("version", lua.push_string(build_info::PKG_VERSION));
    set_opt_str_field!("commit", build_info::GIT_COMMIT_HASH);
    set_opt_str_field!("commit_short", build_info::GIT_COMMIT_HASH_SHORT);
    match build_info::GIT_DIRTY {
        Some(dirty) => lua.push_boolean(dirty),
        None => lua.push_nil(),
    }
    lua.set_field(-2, lua_string!("dirty"));
    set_field!("built_at", lua.push_string(build_info::BUILT_TIME_UTC));
    set_field!("target", lua.push_string(build_info::TARGET));
    set_field!("rustc_version", lua.push_string(build_info::RUSTC_VERSION));
    set_field!("realm", lua.push_string(REALM));
    // True when built by a recognised CI platform (currently: GitHub Actions),
    // as opposed to a local `cargo build`. See README for what this promises.
    set_field!(
        "official",
        lua.push_boolean(build_info::CI_PLATFORM.is_some())
    );
    set_field!("repository", lua.push_string(env!("BUILD_REPOSITORY")));
    set_field!("run_url", lua.push_string(env!("BUILD_RUN_URL")));

    1
}

#[lua_function]
unsafe fn newindex(lua: State) -> i32 {
    lua.error(err!("environment variables cannot be set"));
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export {
        ($func:ident, $name:literal) => {
            lua.push_function($func);
            lua.set_field(-2, lua_string!($name));
        };
        ($value:literal, $name:literal) => {
            lua.push_string($value);
            lua.set_field(-2, lua_string!($name));
        };
    }

    #[cfg(feature = "gmcl")]
    {
        override_stdout();
    }

    // Build the function table up front, rather than lazily on first lookup.
    LazyLock::force(&FUNC_MAP);

    // Create _G.environ metatable
    lua.new_metatable(lua_string!("environ"));
    export!(index, "__index");
    export!(newindex, "__newindex");
    export!("environ", "__name");

    // Lock the metatable: getmetatable(environ) returns `false` instead of
    // the real table, and setmetatable(environ, ...) errors. Without this,
    // any addon could reach in and rewrite __index/__newindex, silently
    // breaking the read-only and real-environment guarantees for everyone
    // else on the server.
    lua.push_boolean(false);
    lua.set_field(-2, lua_string!("__metatable"));

    // Set and pop the metatable.
    lua.new_userdata(0, Some(-1));

    // Set and pop the environ table in the global environment.
    lua.set_global(lua_string!("environ"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    // gmod-rs's #[gmod13_close] appends the gmcl::restore_stdout() that
    // override_stdout requires on unload (or the game crashes), so there is
    // deliberately no explicit call here.
    0
}
