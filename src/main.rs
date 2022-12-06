#![feature(let_else)]

extern crate exitcode;

use std::io::{Error as IoError, stdin};
use std::process::exit;

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
        None => repl(),
        Some(file) => run_file(file),
    };
}

fn repl() {
    loop {
        let Ok(input) = read() else {
            exit(exitcode::USAGE);
        };
        let Ok(result) = eval(&input) else {
            exit(exitcode::USAGE);
        };
        println!("{}", result);
    }
}

fn run_file(path: String) {
    unimplemented!("path={}", path);
}

fn read() -> Result<String, IoError> {
    let mut input = String::new();
    if let Err(e) = stdin().read_line(&mut input) {
        return Err(e);
    }
    Ok(input)
}

struct EvalError;

fn eval(input: &String) -> Result<String, EvalError> {
    Ok(input.clone()) //TODO
}