#![feature(c_unwind)]

#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State};
use gmod::lua_function;

#[macro_use] extern crate gmod;

static MOD_NAME: &str = "environ";
macro_rules! err {
    () => {format!("{} had an error.", MOD_NAME)};
    ($arg:literal) => {format!("{} was unable to {}", MOD_NAME, $arg)};
    ($arg:literal, $err:literal) => {format!("{} was unable to {}: {:?}", MOD_NAME, $arg, $err)};
}

unsafe fn error(lua: State, err: &str){
    lua.error(err);
}

unsafe fn arg_err(lua: State, pos: i16, exp: &str, real: &str){
    error(lua, format!("{}: Bad Argument in position #{}, expected {} got {}", MOD_NAME, pos, exp, real).as_str());
}

#[lua_function]
unsafe fn index(lua: State) -> i32 {
    let arg_type = lua.get_type(-1);
    if (arg_type != "string"){
        arg_err(lua, 1, "string", arg_type);
    }

    let arg = lua.get_string(-1);
    let environ_key: str = match arg {
        None => {
            error(lua, err!("fetch argument string").as_str());
            ""
        },
        Some(arg_str) => {
            *arg_str
        }
    };

    println!("{}: {}", arg_type, environ_key);

    0
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
            // _G.sysinfo.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        };
        ($func:ident, $name:literal) => {
            // _G.sysinfo.$name
            lua.push_function($func);
            lua.set_field(-2, lua_string!($name));
        }
    }

    #[cfg(feature = "gmcl")]{
        override_stdout();
    }

    // Create _G.environ
    lua.create_table(0, 1);
    export_lua_function!(newindex, "__ignore");

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
    println!("Goodbye from binary module!");
    0
}
