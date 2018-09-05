use libq3vm_sys;
pub use libq3vm_sys::CallArgs;
pub use libq3vm_sys::SyscallHandler;
use std::ffi::CString;
use system::Environment;

#[allow(dead_code)]
pub struct Q3VM {
    module: CString,
    bytecode: Vec<u8>,
    handle: libq3vm_sys::VMHandle,
}
/*
    pub fn compile<T: AsRef<Path>, S: AsRef<Path>>(tools_dir: T, syscall_def: S, src: Iter<&str>)
    {

    }
*/
#[macro_export]
macro_rules! vm_compile {
    ($tools:expr, $syscall_def:expr, $target:expr => [ $( $src_file:expr ),* ]) => {
        use $crate::std::process::Command;
        use $crate::std::fs;
        use $crate::std::env;
        use $crate::std::path::PathBuf;
        use $crate::std::fs::File;
        use $crate::std::io::Write;

        let mut asm_files: Vec<String> = Vec::new();
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
        .env("PATH", $tools)
        .args(args)
        .output()
        .expect("Failed to compile script source code");
        println!("{}",String::from_utf8_lossy(&result.stdout));
        println!("{}",String::from_utf8_lossy(&result.stderr));
        asm_files.push(asm_file.clone());
)*
        let syscall_file = fs::canonicalize($syscall_def).expect("Failed to find syscall definitions");
        let mut script = PathBuf::from(&asm_files[0]).parent().unwrap().to_path_buf();
        script.push("game.q3asm");
        let linker_script_path = String::from(script.to_str().unwrap());
        let mut file = File::create(&linker_script_path).expect("Failed to create assembler script");
        file.write(format!("-o \"{}\"\n",$target).as_bytes());
        file.write(format!("{}\n", syscall_file.to_str().expect("Non valid UTF-8 path provied")).as_bytes());
        for asm in asm_files {
            file.write(format!("{}\n", asm).as_bytes());
        }
        file.sync_all().unwrap();

        let result = Command::new("q3asm")
        .env("PATH", $tools)
        .args(&["-f",&linker_script_path])
        .output()
        .expect("Failed to assemble byte code");
        println!("{}",String::from_utf8_lossy(&result.stdout));
        println!("{}",String::from_utf8_lossy(&result.stderr));
    }
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
