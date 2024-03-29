use crate::environment::Environment;
use crate::expression::{BinaryOperator, Expression, Literal, UnaryOperator};
use crate::object;
use crate::object::{Callable, Function, Object};
use crate::parser::*;
use crate::resolver::Resolver;
use crate::result::*;
use crate::scanner::*;
use crate::statement::Statement;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Interpreter {
    globals: Environment,
    pub environment: Environment,
    locals: HashMap<Expression, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut globals = Environment::new();

        #[derive(Debug)]
        struct Clock;
        impl Callable for Clock {
            fn signature(&self) -> String {
                "clock".to_string()
            }
            fn arity(&self) -> usize {
                0
            }
            fn call(
                &self,
                _interpreter: &Interpreter,
                _arguments: Vec<Object>,
            ) -> Result<Object, object::Error> {
                todo!()
            }
        }
        globals.define("clock".to_string(), Object::Function(Rc::new(Clock {})));

        let environment = globals.clone();
        Interpreter {
            globals,
            environment,
            locals: HashMap::new(),
        }
    }

    pub fn new_for_closure(&self, environment: Environment) -> Interpreter {
        Interpreter {
            globals: self.globals.clone(),
            environment,
            locals: self.locals.clone(),
        }
    }

    pub fn repl(mut self) -> Result<(), Error> {
        for line_number in 0..usize::MAX {
            let input = Self::read()?;
            let result = self.eval(&input, line_number);
            if let Err(errors) = result {
                for error in errors {
                    println!("ERROR: {error:?}");
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
                    println!("ERROR in line {line_number}: {error:?}");
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

    fn eval(&mut self, source: &str, line_number: usize) -> Result<(), Vec<Error>> {
        let tokens = match Scanner::new(source, line_number).scan_tokens() {
            Ok(tokens) => tokens,
            Err(error) => return Err(vec![error]),
        };

        let statements = match Parser::new(tokens).parse() {
            Ok(statements) => statements,
            Err(error) => return Err(vec![error]),
        };

        let mut resolver = Resolver::new(self);
        let resolver_errors = statements
            .iter()
            .filter_map(|statement| resolver.resolve_statement(statement).err())
            .collect::<Vec<_>>();

        if !resolver_errors.is_empty() {
            return Err(resolver_errors);
        }

        let errors = statements
            .into_iter()
            .filter_map(|statement| {
                self.execute(statement)
                    .map_err(Error::EvaluationError)
                    .err()
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    pub fn execute(&mut self, statement: Statement) -> Result<(), object::Error> {
        match statement {
            Statement::If {
                condition,
                then_statement,
                else_statement,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(*then_statement)?;
                } else if let Some(statement) = else_statement {
                    self.execute(*statement)?;
                }
            }
            Statement::Print(expression) => {
                println!("{}", self.evaluate(expression)?);
            }
            Statement::Return(expression) => {
                let value = if let Some(expression) = expression {
                    self.evaluate(expression)?
                } else {
                    Object::Nil
                };
                return Err(object::Error::Return(value));
            }
            Statement::Expression(expression) => {
                self.evaluate(expression)?;
            }
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                let value = if let Some(expression) = expression {
                    self.evaluate(expression)?
                } else {
                    Object::Nil
                };
                self.environment.define(identifier, value);
            }
            Statement::FunctionDeclaration(function_declaration) => {
                let identifier = function_declaration.identifier.clone();
                let function = Object::Function(Rc::new(Function::new(
                    function_declaration,
                    self.environment.clone(),
                )));
                self.environment.define(identifier, function)
            }
            Statement::While {
                expression,
                statement,
            } => {
                while self.evaluate(expression.clone())?.is_truthy() {
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

    fn evaluate(&mut self, expression: Expression) -> Result<Object, object::Error> {
        match expression {
            Expression::Literal(literal) => {
                let object = match literal {
                    Literal::Number(number) => Object::Number(number),
                    Literal::String(string) => Object::String(string),
                    Literal::True => Object::Boolean(true),
                    Literal::False => Object::Boolean(false),
                    Literal::Nil => Object::Nil,
                };
                Ok(object)
            }
            Expression::Unary {
                operator,
                expression,
            } => {
                let expresssion_value = self.evaluate(*expression)?;
                match operator {
                    UnaryOperator::Negation => Ok(Object::Boolean(!expresssion_value.is_truthy())),
                    UnaryOperator::Minus => expresssion_value.unary_minus(),
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;
                match operator {
                    BinaryOperator::Equality => Ok(Object::Boolean(left_value == right_value)),
                    BinaryOperator::Different => Ok(Object::Boolean(left_value != right_value)),
                    BinaryOperator::Less => Ok(Object::Boolean(left_value < right_value)),
                    BinaryOperator::EqualOrLess => Ok(Object::Boolean(left_value <= right_value)),
                    BinaryOperator::Greater => Ok(Object::Boolean(left_value > right_value)),
                    BinaryOperator::EqualOrGreater => {
                        Ok(Object::Boolean(left_value >= right_value))
                    }
                    BinaryOperator::Addition => left_value + right_value,
                    BinaryOperator::Subtraction => left_value - right_value,
                    BinaryOperator::Multiplication => left_value * right_value,
                    BinaryOperator::Division => left_value / right_value,
                    BinaryOperator::Or => Ok(if left_value.is_truthy() {
                        left_value
                    } else {
                        right_value
                    }),
                    BinaryOperator::And => Ok(if left_value.is_truthy() {
                        right_value
                    } else {
                        left_value
                    }),
                }
            }
            Expression::Variable(string) => self.look_up_variable(string),
            Expression::Assignment { identifier, value } => {
                let object = self.evaluate(*value.clone())?;
                let depth = self.locals.get(&Expression::Assignment {
                    identifier: identifier.clone(),
                    value,
                });
                match depth {
                    Some(depth) => self.environment.assign_at(*depth, identifier, object),
                    None => self.globals.assign(identifier, object),
                }
            }
            Expression::Grouping(expression) => self.evaluate(*expression),
            Expression::FunctionCall {
                function,
                arguments,
            } => {
                let function_object = self.evaluate(*function)?;
                let Object::Function(function) = function_object else {
                    return Err(object::Error::AttemptedToCallUncallableExpression{ called: function_object });
                };
                if arguments.len() != function.arity() {
                    return Err(object::Error::WrongNumberOfArguments {
                        expected: function.arity(),
                        actual: arguments.len(),
                    });
                }
                let arguments = arguments
                    .into_iter()
                    .map(|arg| self.evaluate(arg))
                    .collect::<Result<Vec<Object>, object::Error>>()?;
                function.call(&self, arguments)
            }
        }
    }

    pub fn resolve(&mut self, expression: Expression, depth: usize) {
        self.locals.insert(expression, depth);
    }

    fn look_up_variable(&self, variable_name: String) -> Result<Object, object::Error> {
        let depth = self
            .locals
            .get(&Expression::Variable(variable_name.clone()));
        match depth {
            Some(depth) => self.environment.get_at(*depth, &variable_name),
            None => self.globals.get(&variable_name),
        }
    }
}
