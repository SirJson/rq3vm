#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive_derive;
extern crate libq3vm_sys;
extern crate num_traits;
extern crate pretty_env_logger;
extern crate regex;
extern crate strfmt;
#[macro_use]
extern crate log;

mod system;
mod vm;

use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    pretty_env_logger::init_custom_env("info");
    let filepath = "./example/rustic.qvm";
    let mut bytecode: Vec<u8> = Vec::new();
    let mut vm_file = File::open(filepath)?;
    vm_file.read_to_end(&mut bytecode)?;
    let mut vm = vm::Q3VM::new(filepath, bytecode, system::call_handler);
    vm.call(0, &[]);
    Ok(())
}
