#![feature(let_else)]

extern crate exitcode;

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
    use std::process::exit;

    let args = Args::parse();
    if let Err(error) = match args.file_path {
        None => repl(),
        Some(file) => run_file(file),
    } {
        exit(error.exit_code());
    }
}

fn repl() -> Result<()> {
    loop {
        let input = read()?;
        let result = eval(&input)?;
        println!("{}", result);
    }
}

fn run_file(path: String) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let result = eval(&line?)?;
        println!("{}", result);
    }

    Ok(())
}

fn read() -> Result<String> {
    use std::io::{stdin, stdout, Write};

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
