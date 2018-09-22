use libq3vm_sys::CallArgs;

pub fn str_to_int(mut args: CallArgs) -> i32 {
    match args.string(1).parse::<i32>() {
        Ok(val) => val,
        Err(error) => {
            error!("String to int convert failed: {:?}", error);
            0
        }
    }
}

pub fn syscode_fault(func: isize) -> isize {
    error!("Invalid or unknown syscall code: {}", func);
    -1
}
