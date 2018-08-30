use std::path::Path;
use std::process::Command;

fn main() {
    let output = Command::new("make")
        .current_dir(Path::new("./example"))
        .output()
        .expect("failed to execute process");
    if !output.status.success() {
        println!("cargo:warning=Building example QVM failed!");
        println!(
            "cargo:warning={}",
            String::from_utf8(output.stdout).unwrap_or_default()
        );
        println!(
            "cargo:warning={}",
            String::from_utf8(output.stderr).unwrap_or_default()
        );
    }
}
