#![feature(c_unwind)]

use std::collections::HashMap;
use std::env;
#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State};
use gmod::lua_function;
use lazy_static::lazy_static;

#[macro_use] extern crate gmod;
#[macro_use] extern crate debug_print;

#[cfg(not(windows))]
const PATH_SEP: &str = ":";
#[cfg(windows)]
const PATH_SEP: &str = ";";

type RustLuaFunction = unsafe extern "C-unwind" fn(State) -> i32;
lazy_static! {
    static ref FUNC_MAP: HashMap<&'static str, RustLuaFunction> = {
        let mut m = HashMap::new();

        macro_rules! export {
            ($name:ident) => {
                m.insert(stringify!($name), $name as RustLuaFunction);
            };
            ($func:ident, $name:literal) => {
                m.insert($name, $func as RustLuaFunction);
            }
        }

        export!(get_path);
        export!(get_csv);

        m
    };
}

/// Get the requested string index, agnostic of method call type.
///
/// If we're called as a colon method:
/// `environ:whatever("env_key")` -> `environ.whatever(environ, "env_key")` -> `(table, string)`
///
/// Whereas, if we're called as a dot method:
/// `environ.whatever("env_key")` -> `string`
///
/// I don't care which is done, so we support both.
/// However, documentation will only show dot methoding.
macro_rules! requested_index {
    ( $lua:ident ) => {
        {
            let t = $lua.get_type(1);
            debug_println!("{}", t);

            let str_key = if (t == "table" || t == "UserData") {
                debug_println!("fetched as a colon method");
                $lua.check_string(2)
            } else {
                debug_println!("fetched as a dot method");
                $lua.check_string(1)
            };

            str_key
        }
    }
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
                    debug_println!("{} -> {}: {}", env!("CARGO_CRATE_NAME"), str_idx, val);
                    lua.push_string(val.as_str())
                }
                Err(err) => {
                    debug_println!("{} -> {} failed: {}", env!("CARGO_CRATE_NAME"), str_idx, err);
                    lua.push_nil();
                }
            }

            1
        }
    };

    rtn
}

unsafe fn push_table(lua: State, split: Vec<&str>){
    lua.create_table(split.len() as i32, 0);
    let mut i = 0;
    for s in split {
        let s = s.trim();
        if s != "" {
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
        },
        Err(err) => {
            debug_println!("{} -> {}: {}", env!("CARGO_CRATE_NAME"), "PATH", err);
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
            debug_println!("{} -> {}: {}", env!("CARGO_CRATE_NAME"), str_idx, val);
            let val = val.as_str();
            let split = val.split(",").collect::<Vec<&str>>();
            push_table(lua, split);
        }
        Err(err) => {
            debug_println!("{} -> {} failed: {}", env!("CARGO_CRATE_NAME"), str_idx, err);
            lua.new_table();
        }
    }

    1
}

#[lua_function]
unsafe fn newindex(lua: State) -> i32 {
    lua.error("Environment Variables cannot be set.");
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export {
        ($name:ident) => {
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        };
        ($func:ident, $name:literal) => {
            lua.push_function($func);
            lua.set_field(-2, lua_string!($name));
        };
        ($value:literal, $name:literal) => {
            lua.push_string($value);
            lua.set_field(-2, lua_string!($name));
        }
    }

    #[cfg(feature = "gmcl")]{
        override_stdout();
    }

    // Create _G.environ metatable
    lua.new_metatable(lua_string!("environ"));
    export!(index, "__index");
    export!(newindex, "__newindex");
    export!("environ", "__name");

    // Set and pop the metatable.
    lua.new_userdata(0, Some(-1));

    // Set and pop the environ table in the global environment.
    lua.set_global(lua_string!("environ"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    0
}
