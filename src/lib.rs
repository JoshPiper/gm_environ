#![feature(c_unwind)]

use std::env;
#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State};
use gmod::lua_function;

#[macro_use] extern crate gmod;

static MOD_NAME: &str = "environ";

#[cfg(not(windows))]
const PATH_SEP: &str = ":";
#[cfg(windows)]
const PATH_SEP: &str = ";";

unsafe fn error<S: AsRef<str>>(lua: State, err: S){
    lua.error(err);
}

#[lua_function]
unsafe fn index(lua: State) -> i32 {
    let str_idx = lua.check_string(2);
    let env_var = env::var(str_idx.as_ref());

    if env_var.is_err() {
        println!("failed to read: {}", env_var.err().unwrap());
        lua.push_nil();
    } else {
        println!("{} -> {}: {}", MOD_NAME, str_idx, env_var.clone().unwrap());
        lua.push_string(env_var.unwrap().as_str())
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
                if s != "" {
                    i += 1;
                    lua.push_string(s);
                    lua.raw_seti(-2, i);
                }
            }
        },
        Err(err) => {
            println!("{} -> {}: {}", MOD_NAME, "PATH", err);
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
    lua.create_table(0, 1);
    export_lua_function!(get_path);

    // Create _G.environ metatable
    lua.create_table(0, 1);
    export_lua_function!(index, "__index");

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

