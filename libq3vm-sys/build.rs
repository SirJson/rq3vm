// build.rs

extern crate bindgen;
extern crate cc;
extern crate crc;

use crc::crc32;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

const C_SRC: [&'static str; 2] = ["src/vm.c", "src/libq3vm.c"];
const H_SRC: [&'static str; 2] = ["src/libq3vm.h", "src/vm.h"];
const HASH_FILE: &'static str = "./libq3vm.crc32";
const VMLIB_NAME: &'static str = "q3vm";

fn main() {
    let mut crc_results: Vec<u32> = Vec::new();
    let hash_path = HASH_FILE;
    let mut build_clib = false;
    let mut crc_index = 0;
    let out_dir = env::var("OUT_DIR").unwrap();

    for srcfile in C_SRC.iter() {
        let mut file = File::open(srcfile).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        crc_results.push(crc32::checksum_ieee(contents.as_bytes()));
    }

    for srcfile in H_SRC.iter() {
        let mut file = File::open(srcfile).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        crc_results.push(crc32::checksum_ieee(contents.as_bytes()));
    }

    match File::open(hash_path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            let mut bytes_read = reader.read_line(&mut line).unwrap_or(0);
            while bytes_read > 0 {
                let computed_val = crc_results[crc_index];
                let saved_val = line
                    .trim()
                    .parse::<u32>()
                    .expect(&format!("\"{}\" is not a line", line.clone().trim()));
                if computed_val != saved_val {
                    build_clib = true;
                    println!(
                        "cargo:warning=Checksum fail in line {}! {} != {}",
                        crc_index, computed_val, saved_val
                    );
                    break;
                }
                crc_index = crc_index + 1;
                line.truncate(0);
                bytes_read = reader.read_line(&mut line).unwrap_or(0);
            }
        }
        Err(_) => build_clib = true,
    }

    if build_clib {
        println!("cargo:warning=Rebuilding lib{}!", VMLIB_NAME);
        cc::Build::new().files(C_SRC.iter()).compile(VMLIB_NAME);

        let bindings = bindgen::Builder::default()
            .header(H_SRC[0])
            .generate_comments(false)
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        let new_hashes = File::create(hash_path).expect("Failed to create hashing file!");
        let mut buffer = BufWriter::new(new_hashes);
        for i in 0..crc_results.len() {
            buffer
                .write_fmt(format_args!("{}\n", crc_results[i]))
                .unwrap();
        }
    } else {
        println!("cargo:rustc-link-search=native={}", out_dir);
        println!("cargo:rustc-link-lib=static={}", VMLIB_NAME);
    }
}
