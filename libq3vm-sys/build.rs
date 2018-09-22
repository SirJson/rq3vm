// build.rs

// TODO: Does this work on Windows?

extern crate bindgen;
extern crate cc;
extern crate crc;
#[macro_use]
extern crate commandspec;
#[macro_use]
extern crate maplit;

use crc::crc32;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::slice::Iter;

const Q3VM_SRC: [&'static str; 4] = [
    "ext/q3vm/src/vm/vm.h",
    "ext/q3vm/src/vm/vm.c",
    "src/libq3vm.h",
    "src/libq3vm.c",
];

const Q3VM_HASH_FILE: &'static str = "./libq3vm.crc32";
const LCC_HASH_FILE: &'static str = "./lcc.crc32";
const RCC_HASH_FILE: &'static str = "./rcc.crc32";
const CPP_HASH_FILE: &'static str = "./cpp.crc32";
const Q3ASM_HASH_FILE: &'static str = "./q3asm.crc32";

const Q3VM_TARGET: &'static str = "q3vm";

const Q3ASM_SRC_PATH: &'static str = "ext/q3vm/q3asm";
const LCC_BASE_PATH: &'static str = "ext/q3vm/lcc";
const CPP_SRC_DIR: &'static str = "/cpp";
const RCC_SRC_DIR: &'static str = "/src";
const LCC_SRC_DIR: &'static str = "/etc";

const BINARY_DIR: &'static str = "tools";

fn compute_crc32<T>(files: Iter<T>) -> Vec<u32>
where
    T: std::convert::AsRef<std::path::Path>,
{
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

fn get_cpp_src() -> Vec<String> {
    let path = fs::canonicalize(format!("{}{}", LCC_BASE_PATH, CPP_SRC_DIR))
        .expect("Failed to canonicalize cpp source path");
    fs::read_dir(path)
        .expect("Failed to read cpp source path")
        .filter(|pool| !pool.is_err())
        .map(|m| m.unwrap().path())
        .filter(|x| {
            x.extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                == "c"
                || x.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == "h"
        }).map(|f| String::from(f.to_str().unwrap()))
        .collect()
}

fn get_q3asm_src() -> Vec<String> {
    let path = fs::canonicalize(Q3ASM_SRC_PATH).expect("Failed to canonicalize q3asm source path");
    fs::read_dir(path)
        .expect("Failed to read q3asm source path")
        .filter(|pool| !pool.is_err())
        .map(|m| m.unwrap().path())
        .filter(|x| {
            x.extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                == "c"
                || x.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == "h"
        }).map(|f| String::from(f.to_str().unwrap()))
        .collect()
}

fn get_rcc_src() -> Vec<String> {
    let path = fs::canonicalize(format!("{}{}", LCC_BASE_PATH, RCC_SRC_DIR))
        .expect("Failed to canonicalize rcc source path");
    fs::read_dir(path)
        .expect("Failed to read rcc source path")
        .filter(|pool| !pool.is_err())
        .map(|m| m.unwrap().path())
        .filter(|x| {
            x.extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                == "c"
                || x.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == "h"
        }).map(|f| String::from(f.to_str().unwrap()))
        .collect()
}

fn get_lcc_src() -> Vec<String> {
    let path = fs::canonicalize(format!("{}{}", LCC_BASE_PATH, LCC_SRC_DIR))
        .expect("Failed to canonicalize lcc source path");
    fs::read_dir(path)
        .expect("Failed to read lcc source path")
        .filter(|pool| !pool.is_err())
        .map(|m| m.unwrap().path())
        .filter(|x| {
            x.extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                == "c"
                || x.extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    == "h"
        }).map(|f| String::from(f.to_str().unwrap()))
        .collect()
}

fn run_make(path: &str) -> Result<(), commandspec::Error> {
    execute!(
        r"
            cd {path}
            make clean
        ",
        path = path
    )?;
    execute!(
        r"
            cd {path}
            make
        ",
        path = path
    )?;
    Ok(())
}

fn build_q3vm() {
    cc::Build::new()
        .include("ext/q3vm/src/vm")
        .files(Q3VM_SRC.iter())
        .cargo_metadata(false)
        .compile(Q3VM_TARGET);
}

fn generate_bindings() {
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

fn cargo_cp(src: String, target: String) {
    let src_path =
        fs::canonicalize(&src).expect(&format!("Failed to canonicalize source path => '{}'", &src));
    let target_path = PathBuf::from(target); //fs::canonicalize(&target).expect(&format!("Failed to canonicalize target path => {}",&target));

    fs::copy(src_path, target_path).expect("Failed to copy lcc to target directory");
}

fn main() {
    std::fs::create_dir(BINARY_DIR).unwrap_or(()); // If the directory already exists do nothing.
    let out_dir = env::var("OUT_DIR").unwrap();

    let q3asm_src = get_q3asm_src();
    let rcc_src = get_rcc_src();
    let lcc_src = get_lcc_src();
    let cpp_src = get_cpp_src();

    let q3vm_hash = compute_crc32(Q3VM_SRC.iter());

    let toolset_hashes = hashmap!{
        Q3ASM_HASH_FILE => compute_crc32(q3asm_src.iter()),
        RCC_HASH_FILE => compute_crc32(rcc_src.iter()),
        LCC_HASH_FILE => compute_crc32(lcc_src.iter()),
        CPP_HASH_FILE => compute_crc32(cpp_src.iter()),
    };

    if !crc32_file_match(&toolset_hashes[Q3ASM_HASH_FILE], Q3ASM_HASH_FILE) {
        if let Err(error) = run_make(Q3ASM_SRC_PATH) {
            panic!("Failed to build q3asm: {:?}", error)
        }
        cargo_cp(
            format!("{}/q3asm", Q3ASM_SRC_PATH),
            format!("{}/q3asm", BINARY_DIR),
        );
        write_hash_file(Q3ASM_HASH_FILE, &toolset_hashes[Q3ASM_HASH_FILE]);
    }
    if !crc32_file_match(&toolset_hashes[RCC_HASH_FILE], RCC_HASH_FILE)
        || !crc32_file_match(&toolset_hashes[LCC_HASH_FILE], LCC_HASH_FILE)
        || !crc32_file_match(&toolset_hashes[CPP_HASH_FILE], CPP_HASH_FILE)
    {
        if let Err(error) = run_make(LCC_BASE_PATH) {
            panic!("Failed to build lcc: {:?}", error);
        }
        cargo_cp(
            format!("{}/build/lcc", LCC_BASE_PATH),
            format!("{}/lcc", BINARY_DIR),
        );
        cargo_cp(
            format!("{}/build/rcc", LCC_BASE_PATH),
            format!("{}/q3rcc", BINARY_DIR),
        );
        cargo_cp(
            format!("{}/build/cpp", LCC_BASE_PATH),
            format!("{}/q3cpp", BINARY_DIR),
        );
        write_hash_file(RCC_HASH_FILE, &toolset_hashes[RCC_HASH_FILE]);
        write_hash_file(LCC_HASH_FILE, &toolset_hashes[LCC_HASH_FILE]);
        write_hash_file(CPP_HASH_FILE, &toolset_hashes[CPP_HASH_FILE]);
    }
    let bindings_exist = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bindings.rs")
        .exists();
    if !crc32_file_match(&q3vm_hash, Q3VM_HASH_FILE) || !bindings_exist {
        build_q3vm();
        generate_bindings();
        write_hash_file(Q3VM_HASH_FILE, &q3vm_hash);
    }
    link_lib(&out_dir, Q3VM_TARGET);
}
