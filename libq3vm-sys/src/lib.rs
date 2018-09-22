#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw;
use std::slice;
use std::sync::Mutex;

pub type VMHandle = vm_s;
pub type SyscallHandler = fn(function: isize, args: CallArgs, name: String) -> isize;

pub const MAX_VMSYSCALL_ARGS: usize = 16;
pub const MAX_VMMAIN_ARGS: usize = 13;

lazy_static! {
    static ref SYSCALL_MAP: Mutex<SyscallMap> = Mutex::new(SyscallMap::init());
}

pub struct SyscallMap {
    data: HashMap<CString, SyscallHandler>,
}

impl SyscallMap {
    pub fn init() -> SyscallMap {
        SyscallMap {
            data: HashMap::new(),
        }
    }

    pub fn handler(&self, name: &CString) -> &SyscallHandler {
        &self.data[name]
    }

    pub fn add(&mut self, name: &CString, handler: SyscallHandler) {
        self.data.insert(name.clone(), handler);
    }

    pub fn remove(&mut self, name: &CString) {
        self.data.remove(name);
    }
}

pub struct CallArgs<'a> {
    args: &'a [isize],
    vm: *mut VMHandle,
}

impl<'a> CallArgs<'a> {
    pub fn new(vm: *mut VMHandle, argv: *mut isize) -> Self {
        unsafe {
            CallArgs {
                args: slice::from_raw_parts(argv, MAX_VMSYSCALL_ARGS),
                vm,
            }
        }
    }

    pub fn i32(&self, idx: usize) -> i32 {
        if idx >= MAX_VMSYSCALL_ARGS {
            warn!("Tried to access arg that is out of bounds. Max supported vm syscall args = {} but arg {} was requested",MAX_VMSYSCALL_ARGS,idx);
            return 0;
        }
        self.args[idx] as i32
    }

    pub fn iptr(&self, idx: usize) -> isize {
        if idx >= MAX_VMSYSCALL_ARGS {
            warn!("Tried to access arg that is out of bounds. Max supported vm syscall args = {} but arg {} was requested",MAX_VMSYSCALL_ARGS,idx);
            return 0;
        }
        self.args[idx]
    }

    pub fn string(&mut self, idx: usize) -> String {
        if idx >= MAX_VMSYSCALL_ARGS {
            warn!("Tried to access arg that is out of bounds. Max supported vm syscall args = {} but arg {} was requested",MAX_VMSYSCALL_ARGS,idx);
            return String::default();
        }
        unsafe {
            let cstr = CStr::from_ptr(VMA_(self.args[idx], self.vm) as *mut raw::c_char);
            match cstr.to_str() {
                Ok(val) => String::from(val),
                Err(error) => {
                    match CString::new(format!("VM error: {:?}", error)) {
                        Ok(cstring) => {
                            error!("VM Error: no string found at argument index {}", idx);
                            Com_Error(
                                vmErrorCode_t::VM_JUMP_TO_INVALID_INSTRUCTION,
                                cstring.as_ptr(),
                            );
                        }
                        Err(error) => {
                            error!("Fatal VM Error: String convertion failed: {:?}", error);
                        }
                    }

                    String::from("INVALID STRING")
                }
            }
        }
    }
}

extern "C" fn syscalls(arg1: *mut VMHandle, arg2: *mut isize) -> isize {
    {
        let args = CallArgs::new(arg1, arg2);
        let id = -1 - args.iptr(0);
        unsafe {
            let name = vm_name(
                arg1.as_ref()
                    .expect("Failed to get reference to VM module while executing syscall"),
            );
            SYSCALL_MAP.lock().unwrap().handler(&name)(
                id,
                args,
                String::from(name.to_str().unwrap()),
            )
        }
    }
}

pub fn call(handle: &mut VMHandle, cmd: i32) {
    unsafe {
        VM_Call(&mut *(handle), cmd);
    }
}

pub fn create(
    mut module: &CString,
    mut bytecode: &mut Vec<u8>,
    handler: SyscallHandler,
) -> VMHandle {
    unsafe {
        let mut vm: VMHandle = std::mem::uninitialized();
        VM_Create(
            &mut vm,
            (&mut module).as_ptr(),
            (&mut bytecode).as_mut_ptr(),
            bytecode.len() as raw::c_int,
            Some(syscalls),
        );
        SYSCALL_MAP.lock().unwrap().add(module, handler);
        vm
    }
}

pub fn vm_name(handle: &VMHandle) -> CString {
    let data: &[i8] = &handle.name;
    let converted: &[u8] = unsafe { slice::from_raw_parts(data.as_ptr() as *const u8, data.len()) };
    CString::new(
        converted
            .iter()
            .cloned()
            .filter(|x| *x != 0)
            .collect::<Vec<u8>>(),
    ).unwrap_or_else(|err| {
        error!("Failed to convert C string literal from VM: {:?}", err);
        CString::default()
    })
}

pub fn destroy(handle: &mut VMHandle) {
    unsafe {
        {
            let name = vm_name(handle);
            info!("Unloading VM \"{}\"...", &name.to_str().unwrap_or_default());
            if name.as_bytes().len() <= 0 {
                warn!("Invalid VM module detected. 'name' is missing. The system call map will not be cleaned up!");
            } else {
                SYSCALL_MAP.lock().unwrap().remove(&name);
            }
        }
        VM_Free(&mut *(handle));
    }
}
