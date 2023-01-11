use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::evaluate::*;
use crate::parser::*;
use crate::result::*;
use crate::scanner::*;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn repl(mut self) -> Result<()> {
        for line_number in 0..usize::MAX {
            let input = Self::read()?;
            let results = self.eval(&input, line_number);
            for result in results {
                match result {
                    Ok(value) => println!("{}", value),
                    Err(message) => println!("ERROR: {:?}", message),
                }
            }
        }
        Err(Error::OutOfLineNumbers)
    }
    pub fn run_file(mut self, path: String) -> Result<()> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for (line_number, line) in reader.lines().enumerate() {
            let results = self.eval(&line?, line_number);
            for result in results {
                println!("{}", result?);
            }
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

    fn eval(&mut self, source: &String, line_number: usize) -> Vec<Result<String>> {
        let tokens = match Scanner::new(&source, line_number).scan_tokens() {
            Ok(tokens) => tokens,
            Err(error) => return vec![Err(error)],
        };

        let statements = match Parser::new(tokens).parse() {
            Ok(statements) => statements,
            Err(error) => return vec![Err(error)],
        };

        statements
            .into_iter()
            .map(
                |statement| match statement.evaluate(&mut self.environment) {
                    Ok(object) => Ok(object.to_string()),
                    Err(message) => Err(Error::EvaluationError(message)),
                },
            )
            .collect()
    }
}
