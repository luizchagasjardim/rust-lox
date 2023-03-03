use crate::environment::Environment;
use crate::object::{Function, Object};
use crate::parser::*;
use crate::result::*;
use crate::scanner::*;
use std::rc::Rc;

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Environment::new();

        #[derive(Clone, Debug, PartialEq)]
        struct Clock;
        impl Function for Clock {
            fn arity(&self) -> usize {
                0
            }
            fn call(&mut self, interpreter: &Interpreter, arguments: Vec<Object>) -> Object {
                todo!()
            }
        }
        globals.define("clock".to_string(), Object::Function(Rc::new(Clock {})));

        let environment = globals.clone();
        Interpreter {
            globals,
            environment,
        }
    }

    pub fn repl(mut self) -> Result<(), Error> {
        for line_number in 0..usize::MAX {
            let input = Self::read()?;
            let result = self.eval(&input, line_number);
            if let Err(errors) = result {
                for error in errors {
                    println!("ERROR: {:?}", error);
                }
            }
        }
        Err(Error::OutOfLineNumbers)
    }
    pub fn run_file(mut self, path: String) -> Result<(), Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for (line_number, line) in reader.lines().enumerate() {
            let result = self.eval(&line?, line_number);
            if let Err(errors) = result {
                for error in errors {
                    println!("ERROR in line {}: {:?}", line_number, error);
                }
            }
        }
        Ok(())
    }

    fn read() -> Result<String, Error> {
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

    fn eval(&mut self, source: &String, line_number: usize) -> Result<(), Vec<Error>> {
        let tokens = match Scanner::new(&source, line_number).scan_tokens() {
            Ok(tokens) => tokens,
            Err(error) => return Err(vec![error]),
        };

        let statements = match Parser::new(tokens).parse() {
            Ok(statements) => statements,
            Err(error) => return Err(vec![error]),
        };

        let errors = statements
            .into_iter()
            .filter_map(|statement| {
                self.environment
                    .execute(statement)
                    .map_err(Error::EvaluationError)
                    .err()
            })
            .collect::<Vec<_>>();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
