#![feature(let_else)]

extern crate exitcode;

use std::io::{stdin, stdout, Write};
use std::process::exit;

use clap::Parser;

mod result;
use result::*;

/// Lox interpreter written in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to be executed
    file_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    let res = match args.file_path {
        None => repl(),
        Some(file) => run_file(file),
    };
    exit(exit_code(res));
}

fn repl() -> Result<()> {
    loop {
        let input = read()?;
        let result = eval(&input)?;
        println!("{}", result);
    }
}

fn run_file(path: String) -> Result<()> {
    unimplemented!("path={}", path);
}

fn read() -> Result<String> {
    print!(">");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let Some(input) = input.lines().next() else {
        return Err(Error::KeyboardInterrupt);
    };
    Ok(input.into())
}

fn eval(input: &String) -> Result<String> {
    Ok(input.clone()) //TODO
}
