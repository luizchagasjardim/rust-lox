use crate::environment::Environment;
use crate::object;
use crate::object::{Function, Object};
use crate::parser::*;
use crate::result::*;
use crate::scanner::*;
use crate::statement::Statement;
use std::rc::Rc;

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Environment::new();

        #[derive(Debug)]
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
                self.execute(statement)
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

    fn execute(&mut self, statement: Statement) -> Result<(), object::Error> {
        match statement {
            Statement::If {
                condition,
                then_statement,
                else_statement,
            } => {
                if self.environment.evaluate(condition)?.is_truthy() {
                    self.execute(*then_statement)?;
                } else {
                    if let Some(statement) = else_statement {
                        self.execute(*statement)?;
                    }
                }
            }
            Statement::Print(expression) => {
                println!("{}", self.environment.evaluate(expression)?);
            }
            Statement::Expression(expression) => {
                self.environment.evaluate(expression)?;
            }
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                let value = if let Some(expression) = expression {
                    self.environment.evaluate(expression)?
                } else {
                    Object::Nil
                };
                self.environment.define(identifier, value.clone());
            }
            Statement::FunctionDeclaration {
                identifier,
                parameters,
                body,
            } => {
                todo!()
            }
            Statement::While {
                expression,
                statement,
            } => {
                while self.environment.evaluate(expression.clone())?.is_truthy() {
                    self.execute(*statement.clone())?;
                }
            }
            Statement::Block(statements) => {
                self.environment = self.environment.new_child();
                for statement in statements {
                    self.execute(statement)?;
                }
                self.environment = self.environment.end().unwrap();
            }
        };
        Ok(())
    }
}
