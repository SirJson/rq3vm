// build.rs

// TODO: Build LCC!

extern crate bindgen;
extern crate cc;
extern crate crc;

use crc::crc32;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::slice::Iter;

const Q3VM_SRC: [&'static str; 4] = [
    "src/libq3vm.h",
    "ext/q3vm/src/vm/vm.h",
    "ext/q3vm/src/vm/vm.c",
    "src/libq3vm.c",
];
const Q3VM_HASH_FILE: &'static str = "./libq3vm.crc32";
const Q3VM_NAME: &'static str = "q3vm";
//const LCC_HASH_FILE: &'static str = "./lcc.crc32";
//const LCC_BIN: &'static str = "./ext/bin";

fn cargo_print(msg: std::fmt::Arguments) {
    println!("cargo:warning={}", msg);
}

fn compute_crc32(files: Iter<&str>) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    for srcfile in files {
        let mut file = File::open(srcfile).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        results.push(crc32::checksum_ieee(contents.as_bytes()));
    }
    results
}

fn crc32_file_match(data: &Vec<u32>, file_str: &str) -> bool {
    let mut index = 0;
    match File::open(file_str) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            let mut bytes_read = reader.read_line(&mut line).unwrap_or(0);
            while bytes_read > 0 {
                let computed_val = data[index];
                let saved_val = line
                    .trim()
                    .parse::<u32>()
                    .expect(&format!("\"{}\" is not a line", line.clone().trim()));
                if computed_val != saved_val {
                    cargo_print(format_args!(
                        "Checksum fail in line {}! {} != {}",
                        index, computed_val, saved_val
                    ));
                    return false;
                }
                index = index + 1;
                line.truncate(0);
                bytes_read = reader.read_line(&mut line).unwrap_or(0);
            }
            true
        }
        Err(_) => false,
    }
}

fn write_hash_file(file: &str, crc32: &Vec<u32>) {
    let new_hashes = File::create(file).expect("Failed to create hashing file!");
    let mut buffer = BufWriter::new(new_hashes);
    for i in 0..crc32.len() {
        buffer.write_fmt(format_args!("{}\n", crc32[i])).unwrap();
    }
}

fn build_q3vm() {
    cargo_print(format_args!("Rebuilding lib{}!", Q3VM_NAME));
    cc::Build::new()
        .include("src/q3vm/src/vm")
        .files(Q3VM_SRC.iter())
        .compile(Q3VM_NAME);

    let bindings = bindgen::Builder::default()
        .header(Q3VM_SRC[0])
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn link_lib(out_dir: &str, name: &str) {
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", name);
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let q3vm_hash = compute_crc32(Q3VM_SRC.iter());

    if !crc32_file_match(&q3vm_hash, Q3VM_HASH_FILE) {
        build_q3vm();
        write_hash_file(Q3VM_HASH_FILE, &q3vm_hash);
    } else {
        link_lib(&out_dir, Q3VM_NAME);
    }
}
