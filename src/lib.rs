extern crate libq3vm_sys;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod vm;

pub use vm::exec_env;
pub use vm::stdlib;
pub use vm::CallArgs;
pub use vm::SyscallHandler;
pub use vm::Q3VM;
