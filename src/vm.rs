use libq3vm_sys;
pub use libq3vm_sys::CallArgs;
pub use libq3vm_sys::SyscallHandler;
use std::ffi::CString;

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
        Q3VM {
            module,
            bytecode,
            handle,
        }
    }

    pub fn call(&mut self, command: i32) {
        libq3vm_sys::call(&mut self.handle, command);
    }
}

impl Drop for Q3VM {
    fn drop(&mut self) {
        libq3vm_sys::destroy(&mut self.handle);
    }
}
