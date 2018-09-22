use libq3vm_sys;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type ExecEnv = Arc<Mutex<Environment>>;

lazy_static! {
    pub static ref ENVIONRMENTS: Mutex<HashMap<String, ExecEnv>> = Mutex::new(HashMap::new());
}

pub struct Environment {
    allocated_strings: Vec<String>,
}

impl Environment {
    pub fn add(module_name: &str) {
        &mut ENVIONRMENTS.lock().unwrap().insert(
            String::from(module_name),
            Arc::new(Mutex::new(Environment {
                allocated_strings: Vec::new(),
            })),
        );
    }

    fn alloc_managed_str(&mut self, mut args: libq3vm_sys::CallArgs) -> isize {
        self.allocated_strings.push(String::from(args.string(1)));
        (self.allocated_strings.len() - 1) as isize
    }

    fn free_managed_str(&mut self, args: libq3vm_sys::CallArgs) -> isize {
        self.allocated_strings.remove(args.i32(1) as usize);
        0
    }
}
