use libq3vm_sys::{self, VMHandle};
pub use libq3vm_sys::{CallArgs, SyscallHandler};
use std::ffi::CString;

pub mod exec_env;
pub mod stdlib;

use exec_env::{Environment, ExecEnv};

#[macro_export]
macro_rules! vm_compile {
    ($tools:expr, $syscall_def:expr, $target:expr => [ $( $src_file:expr ),* ]) => {
        use ::std::process::Command;
        use ::std::fs;
        use ::std::env;
        use ::std::path::PathBuf;
        use ::std::fs::File;
        use ::std::io::Write;

        fn print_byte_str(data: Vec<u8>) {
            match String::from_utf8(data) {
                Ok(msg) => {
                    if msg.len() > 0 {
                        println!("{}", msg);
                    }
                }
                Err(err) => eprintln!("Failed to read compiler message: {:?}", err),
            }
        }

        let mut asm_files: Vec<String> = Vec::new();
        let tools_path = fs::canonicalize($tools).expect("Failed to canonicalize tools path");
$(
        let src_path = fs::canonicalize($src_file).expect("Failed to find file");
        fs::create_dir("vm_obj").unwrap_or(());
        let asm_file = format!("{}/vm_obj/{}.asm",env::current_dir().unwrap().to_str().unwrap(),$src_file);
        let asm_dir = PathBuf::from(&asm_file);
        fs::create_dir_all(asm_dir.parent().unwrap()).unwrap_or(());
        let src_str = src_path.to_str().expect("Non valid UTF-8 path provied");
        let args = &["-DQ3_VM", "-S", "-Wf-target=bytecode", "-Wf-g", "-o", &asm_file, src_str];
        println!("Compiling: {}...", $src_file);
        let result = Command::new("lcc")
        .env("PATH", &tools_path)
        .args(args)
        .output()
        .expect("Failed to compile script source code");

        print_byte_str(result.stdout);
        print_byte_str(result.stderr);

        asm_files.push(asm_file.clone());
)*
        let syscall_file = fs::canonicalize($syscall_def).expect("Failed to find syscall definitions");
        let mut script = PathBuf::from(&asm_files[0]).parent().unwrap().to_path_buf();
        script.push("game.q3asm");
        let linker_script_path = String::from(script.to_str().unwrap());
        let mut file = File::create(&linker_script_path).expect("Failed to create assembler script");
        file.write(format!("-o \"{}\"\n",$target).as_bytes()).expect("Failed to write line 1 of the assembler script. Do you have the permissions to write in that folder?");
        file.write(format!("{}\n", syscall_file.to_str().expect("Non valid UTF-8 path provied")).as_bytes()).expect("Failed to write line 2 of the assembler script. Do you have the permissions to write in that folder?");
        for asm in asm_files {
            file.write(format!("{}\n", asm).as_bytes()).expect("Failed to write a line of the assembler script. Do you have the permissions to write in that folder?");
        }
        file.sync_all().unwrap();

        let result = Command::new("q3asm")
        .env("PATH", &tools_path)
        .args(&["-f",&linker_script_path])
        .output()
        .expect("Failed to assemble byte code");

        print_byte_str(result.stdout);
        print_byte_str(result.stderr);
    }
}

#[allow(dead_code)]
pub struct Q3VM {
    module: CString,
    bytecode: Vec<u8>,
    handle: VMHandle,
}

impl Q3VM {
    pub fn new(module_name: &str, code: Vec<u8>, syscalls: SyscallHandler) -> Self {
        let module = CString::new(module_name).unwrap();
        let mut bytecode = code;
        let handle = libq3vm_sys::create(&module, &mut bytecode, syscalls);
        Environment::add(module_name);
        Q3VM {
            module,
            bytecode,
            handle,
        }
    }

    pub fn get_exec_env(module: String) -> ExecEnv {
        let env_store = &mut exec_env::ENVIONRMENTS.lock().unwrap();
        env_store
            .get_mut(&module)
            .expect("Missing environment for VM!")
            .clone()
    }

    pub fn call(&mut self, command: i32, args: &[i32]) {
        if args.len() > libq3vm_sys::MAX_VMMAIN_ARGS {
            error!("Tried to call VM with more than supported arguments. vmMain() only supports = {} arguments but you tried to send {}", libq3vm_sys::MAX_VMMAIN_ARGS, args.len());
            return;
        }
        libq3vm_sys::call(&mut self.handle, command);
    }
}

impl Drop for Q3VM {
    fn drop(&mut self) {
        libq3vm_sys::destroy(&mut self.handle);
    }
}
