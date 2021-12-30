#![feature(c_unwind)]

use std::borrow::Borrow;

#[cfg(feature = "gmcl")]
use gmod::gmcl::override_stdout;
use gmod::lua::{State, LuaInt};
use gmod::lua_function;
use sysinfo::{System, SystemExt};
use lazy_static::initialize;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate gmod;

static MOD_NAME: &str = "environ";
macro_rules! err {
    () => {format!("{} had an error.", MOD_NAME)};
    ($arg:literal) => {format!("{} was unable to {}", MOD_NAME, $arg)};
    ($arg:literal, $err:literal) => {format!("{} was unable to {}: {:?}", MOD_NAME, $arg, $err)};
}

lazy_static! {
    static ref SYSTEM: System = System::new_all();
    static ref CORES: usize = match SYSTEM.physical_core_count(){
        Some(cores) => cores,
        None => 0
    };
    static ref TOTAL_MEMORY: u64 = SYSTEM.total_memory();
    static ref TOTAL_SWAP: u64 = SYSTEM.total_swap();
    static ref SYS_NAME: String = match SYSTEM.name(){
        Some(name) => name,
        None => "".to_string()
    };
    static ref OS_LONG_VERSION: String = match SYSTEM.long_os_version(){
        Some(name) => name,
        None => "".to_string()
    };
    static ref OS_VERSION: String = match SYSTEM.os_version(){
        Some(name) => name,
        None => "".to_string()
    };
    static ref KERNEL_VERSION: String = match SYSTEM.kernel_version(){
        Some(name) => name,
        None => "".to_string()
    };
    static ref HOST_NAME: String = match SYSTEM.host_name(){
        Some(name) => name,
        None => "".to_string()
    };
}

unsafe fn error(lua: State, err: String){
    lua.get_global(lua_string!("error"));
    lua.push_string(err.borrow());
    lua.call(1, 0);
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    macro_rules! export_lua_function {
        ($name:ident) => {
            // _G.sysinfo.$name
            lua.push_function($name);
            lua.set_field(-2, concat!(stringify!($name), "\0").as_ptr() as *const i8);
        }
    }

    #[cfg(feature = "gmcl")]{
        override_stdout();
    }

    // Create _G.environ
    lua.create_table(0, 0);
    lua.set_global(lua_string!("environ"));

    0
}

#[gmod13_close]
fn gmod13_close(_lua: State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
