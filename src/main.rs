extern crate exitcode;

use std::process::exit;
use std::env::args;

use clap::Parser;

/// Lox interpreter written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to be executed
    file_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.file_path {
        None => run_prompt(),
        Some(file) => run_file(file),
    };
}

fn run_prompt() {
    unimplemented!();
}

fn run_file(path: String) {
    unimplemented!("path={}", path);
}
