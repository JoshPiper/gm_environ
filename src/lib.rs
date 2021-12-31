#![feature(c_unwind)]

use std::env;
#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State};
use gmod::lua_function;

#[macro_use] extern crate gmod;
#[macro_use] extern crate debug_print;

static MOD_NAME: &str = "environ";

#[cfg(not(windows))]
const PATH_SEP: &str = ":";
#[cfg(windows)]
const PATH_SEP: &str = ";";

unsafe fn error<S: AsRef<str>>(lua: State, err: S){
    lua.error(err);
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

            let str_key = if t == "table" {
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
    let env_var = env::var(str_idx.as_ref());
    match env_var {
        Ok(val) => {
            debug_println!("{} -> {}: {}", MOD_NAME, str_idx, val);
            lua.push_string(val.as_str())
        }
        Err(err) => {
            debug_println!("{} -> {} failed: {}", MOD_NAME, str_idx, err);
            lua.push_nil();
        }
    }

    1
}

#[lua_function]
unsafe fn get_path(lua: State) -> i32 {
    let env_var = env::var("PATH");
    match env_var {
        Ok(val) => {
            let val = val.as_str();
            let split = val.split(PATH_SEP).collect::<Vec<&str>>();
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
        },
        Err(err) => {
            debug_println!("{} -> {}: {}", MOD_NAME, "PATH", err);
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
        Err(err) => {
            debug_println!("{} -> {} failed: {}", MOD_NAME, str_idx, err);
            lua.new_table();
        }
    }

    1
}

#[lua_function]
unsafe fn newindex(lua: State) -> i32 {
    error(lua, "Environment Variables cannot be set.");
    0
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.environ.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        };
        ($func:ident, $name:literal) => {
            // _G.environ.$name
            lua.push_function($func);
            lua.set_field(-2, lua_string!($name));
        }
    }

    #[cfg(feature = "gmcl")]{
        override_stdout();
    }

    // Create _G.environ
    lua.create_table(0, 2);
    export_lua_function!(get_path);
    export_lua_function!(get_csv);

    // Create _G.environ metatable
    lua.create_table(0, 2);
    export_lua_function!(index, "__index");
    export_lua_function!(newindex, "__newindex");

    // Set and pop the metatable.
    lua.set_metatable(-2);

    // Set and pop the environ table in the global environment.
    lua.set_global(lua_string!("environ"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    0
}
