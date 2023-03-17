#![feature(assert_matches)]
#![feature(iter_intersperse)]
extern crate exitcode;

use clap::Parser as ClapParser;

mod environment;
mod expression;
mod interpreter;
mod map_stack;
mod object;
mod parser;
mod resolver;
mod result;
mod scanner;
mod statement;
mod token;

/// Lox interpreter written in Rust
#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to be executed
    file_path: Option<String>,
}

fn main() {
    use interpreter::Interpreter;
    use std::process::exit;

    let args = Args::parse();

    let interpreter = Interpreter::new();

    let program_result = match args.file_path {
        None => interpreter.repl(),
        Some(file) => interpreter.run_file(file),
    };

    if let Err(error) = program_result {
        println!("{error:?}");
        exit(error.exit_code());
    }
}
