use num_traits::FromPrimitive;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use strfmt::strfmt;
use vm;

lazy_static! {
    static ref FMT_TOKEN: Regex = Regex::new(r"[{]\d[}]").unwrap();
    static ref ENVIONRMENTS: Mutex<HashMap<String, Environment>> = Mutex::new(HashMap::new());
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

pub struct Environment {
    allocated_strings: Vec<String>,
}

impl Environment {
    pub fn add(module_name: &str) {
        &mut ENVIONRMENTS.lock().unwrap().insert(
            String::from(module_name),
            Environment {
                allocated_strings: Vec::new(),
            },
        );
    }

    fn syscode_fault(&self, func: isize) -> isize {
        error!("Invalid or unknown syscall code: {}", func);
        -1
    }

    fn print(&self, mut args: vm::CallArgs) -> isize {
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

    fn file_write(&self, mut args: vm::CallArgs) -> isize {
        if let Err(error) = fs::write(args.string(1), args.string(2)) {
            error!("file_write error: {}", error);
        }
        0
    }

    fn str_to_int(&self, mut args: vm::CallArgs) -> i32 {
        match args.string(1).parse::<i32>() {
            Ok(val) => val,
            Err(error) => {
                error!("String to integer convert failed: {:?}", error);
                0
            }
        }
    }

    fn str_new(&mut self, mut args: vm::CallArgs) -> isize {
        self.allocated_strings.push(String::from(args.string(1)));
        (self.allocated_strings.len() - 1) as isize
    }

    fn str_free(&mut self, args: vm::CallArgs) -> isize {
        self.allocated_strings.remove(args.i32(1) as usize);
        0
    }

    fn str_unpack(&self, args: vm::CallArgs) -> isize {
        // TODO: Better integration into the ecosystem so they never know they never had a string in the first place
        println!("{}", self.allocated_strings[args.i32(1) as usize]);
        0
    }
}

pub fn call_handler(func: isize, mut args: vm::CallArgs, name: String) -> isize {
    let env_store = &mut ENVIONRMENTS.lock().unwrap();
    let sys_env = env_store
        .get_mut(&name)
        .expect("Missing environment for VM!");
    match SysCode::from_isize(func * -1) {
        Some(SysCode::Panic) => panic!("VM panic: {}", args.string(1)),
        Some(SysCode::Print) => sys_env.print(args),
        Some(SysCode::FileWrite) => sys_env.file_write(args),
        Some(SysCode::FileRead) => unimplemented!(),
        Some(SysCode::ToInt32) => sys_env.str_to_int(args) as isize,
        Some(SysCode::StringNew) => sys_env.str_new(args),
        Some(SysCode::StringFree) => sys_env.str_free(args),
        Some(SysCode::StringLen) => unimplemented!(),
        Some(SysCode::Str) => sys_env.str_unpack(args),
        None => sys_env.syscode_fault(func),
    }
}
