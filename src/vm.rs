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
