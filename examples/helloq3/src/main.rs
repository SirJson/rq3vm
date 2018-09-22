extern crate pretty_env_logger;
#[macro_use]
extern crate rq3vm;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate num_traits;
extern crate regex;
extern crate strfmt;

use rq3vm::Q3VM;
use std::io::{self, Read};

mod demo_api;

fn main() -> Result<(), io::Error> {
    pretty_env_logger::init_custom_env("info");
    vm_compile!("libq3vm-sys/tools","example/g_syscalls.asm","game" => ["example/g_main.c","example/syslib.c","example/app.c"]);

    let filepath = "helloq3.qvm";

    let mut bytecode: Vec<u8> = Vec::new();
    let mut vm_file = File::open(filepath)?;

    vm_file.read_to_end(&mut bytecode)?;

    let mut vm = Q3VM::new(filepath, bytecode, demo_api::call_handler);
    vm.call(0, &[]);

    Ok(())
}
