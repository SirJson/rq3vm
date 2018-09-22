use num_traits::FromPrimitive;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

use rq3vm;
use rq3vm::stdlib;
use strfmt::strfmt;

lazy_static! {
    static ref FMT_TOKEN: Regex = Regex::new(r"[{]\d[}]").unwrap();
}

#[derive(Primitive)]
enum SysCode {
    Panic = 1,
    Print = 2,
    FileWrite = 3,
    FileRead = 4,
    ToInt32 = 5,
    StringNew = 6,
    StringFree = 7,
    StringLen = 8,
    Str = 9,
}

fn demo_print(mut args: rq3vm::CallArgs) -> isize {
    let msg = args.string(1);
    let token = FMT_TOKEN.find_iter(&msg);
    let argc = token.count();
    let mut vars = HashMap::new();
    for i in 0..argc {
        // index 0 is the function itself, index 1 is the format message so everything from 2 must be our arguments
        let offset = 2;
        vars.insert(i.to_string(), args.string((i + offset) as usize));
    }
    println!("{}", strfmt(&msg, &vars).unwrap());
    0
}

fn demo_file_write(mut args: rq3vm::CallArgs) -> isize {
    if let Err(error) = fs::write(args.string(1), args.string(2)) {
        error!("file_write error: {}", error);
    }
    0
}

pub fn call_handler(func: isize, mut args: rq3vm::CallArgs, name: String) -> isize {
    match SysCode::from_isize(func * -1) {
        Some(SysCode::Panic) => panic!("VM panic: {}", args.string(1)),
        Some(SysCode::Print) => demo_print(args),
        Some(SysCode::FileWrite) => demo_file_write(args),
        Some(SysCode::FileRead) => unimplemented!(),
        Some(SysCode::ToInt32) => stdlib::str_to_int(args) as isize,
        Some(SysCode::StringNew) => unimplemented!(),
        Some(SysCode::StringFree) => unimplemented!(),
        Some(SysCode::StringLen) => unimplemented!(),
        Some(SysCode::Str) => unimplemented!(),
        None => stdlib::syscode_fault(func),
    }
}
