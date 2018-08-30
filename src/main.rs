extern crate libq3vm_sys;
extern crate strfmt;

mod vm;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read};
use strfmt::strfmt;

const SYS_PRINT: isize = -1;
const SYS_FILE_WRITE: isize = -2;
const SYS_FILE_READ: isize = -3;
const SYS_STR_TO_INT: isize = -4;

fn vm_syscall(func: isize, mut args: vm::CallArgs) -> isize {
    match func {
        SYS_PRINT => {
            let msg = args.string(1);
            let argc = args.i32(2);
            let mut vars = HashMap::new();
            for i in 0..=argc {
                let offset = 3;
                vars.insert(i.to_string(), args.string((i + offset) as usize));
            }
            println!("{}", strfmt(&msg, &vars).unwrap());
            0
        }
        SYS_FILE_WRITE => {
            if let Err(error) = fs::write(args.string(1), args.string(2)) {
                println!("file_write error: {}", error);
            }
            0
        }
        SYS_FILE_READ => unimplemented!(),
        SYS_STR_TO_INT => match args.string(1).parse::<isize>() {
            Ok(val) => val,
            Err(error) => {
                println!("String to integer convert failed: {:?}", error);
                0
            }
        },
        _ => {
            println!("Unknown syscall: {}", func);
            -1
        }
    }
}

fn main() -> Result<(), io::Error> {
    let filepath = "./example/rustic.qvm";
    let mut bytecode: Vec<u8> = Vec::new();
    let mut vm_file = File::open(filepath)?;
    vm_file.read_to_end(&mut bytecode)?;
    let mut vm = vm::Q3VM::new(filepath, bytecode, vm_syscall);
    vm.call(0);
    Ok(())
}
