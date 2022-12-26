#![feature(let_else)]

extern crate exitcode;

use clap::Parser as ClapParser;

mod expression;

mod parser;
use parser::*;

mod result;
use result::*;

mod scanner;
use scanner::*;

mod token;

/// Lox interpreter written in Rust
#[derive(ClapParser, Debug)]
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
        println!("{:?}", error);
        exit(error.exit_code());
    }
}

fn repl() -> Result<()> {
    for line_number in 0..usize::MAX {
        let input = read()?;
        let result = eval(&input, line_number)?;
        println!("{}", result);
    }
    Err(Error::OutOfLineNumbers)
}

fn run_file(path: String) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (line_number, line) in reader.lines().enumerate() {
        let result = eval(&line?, line_number)?;
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

fn eval(source: &String, line_number: usize) -> Result<String> {
    let tokens = Scanner::new(&source, line_number).scan_tokens()?;
    let expression = Parser::new(tokens).parse()?;
    Ok(expression.to_code()) //TODO: return expression result instead of code
}
