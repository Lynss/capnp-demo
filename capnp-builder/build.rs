extern crate capnpc;

use capnpc::RustEdition;
use std::{env, fs};

fn main() {
    env::set_var("OUT_DIR", "src/scheme");
    let mut compiler = capnpc::CompilerCommand::new();
    compiler.src_prefix("assets").edition(RustEdition::Rust2018);
    fs::read_dir("assets").unwrap().for_each(|file| {
        compiler.file(file.unwrap().path().display().to_string());
    });
    compiler.run().unwrap();
}
