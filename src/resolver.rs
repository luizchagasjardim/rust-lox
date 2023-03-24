use crate::expression::Expression;
use crate::interpreter::Interpreter;
use crate::map_stack::MapStack;
use crate::result::Error;
use crate::statement::{FunctionDeclaration, Statement};

#[derive(PartialEq)]
enum VariableStatus {
    Declared,
    Defined,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: MapStack<String, VariableStatus>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: MapStack::new(),
        }
    }

    pub fn resolve_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression(expression) => self.resolve_expression(expression)?,
            Statement::If {
                condition,
                then_statement,
                else_statement,
            } => {
                self.resolve_expression(condition)?;
                self.resolve_statement(then_statement)?;
                if let Some(statement) = else_statement {
                    self.resolve_statement(statement)?
                }
            }
            Statement::Print(expression) => {
                self.resolve_expression(expression)?;
            }
            Statement::Return(expression) => {
                if let Some(expression) = expression {
                    self.resolve_expression(expression)?
                }
            }
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                self.declare(identifier);
                if let Some(initializer) = expression {
                    self.resolve_expression(initializer)?;
                }
                self.define(identifier);
            }
            Statement::FunctionDeclaration(FunctionDeclaration {
                identifier,
                parameters,
                body,
            }) => {
                self.declare(identifier); //TODO: this line makes no difference, right?
                self.define(identifier);
                self.resolve_function(parameters, body)?;
            }
            Statement::While {
                expression,
                statement,
            } => {
                self.resolve_expression(expression)?;
                self.resolve_statement(statement)?;
            }
            Statement::Block(statements) => {
                self.begin_scope();
                for statement in statements {
                    self.resolve_statement(statement)?;
                }
                self.end_scope();
            }
        }
        Ok(())
    }

    fn resolve_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::Literal(_) => {}
            Expression::Unary {
                operator,
                expression,
            } => {
                self.resolve_expression(expression)?;
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                self.resolve_expression(left)?;
                self.resolve_expression(right)?;
            }
            Expression::Variable(identifier) => {
                if self.scopes.get_in_top(identifier) == Some(&VariableStatus::Declared) {
                    return Err(todo!());
                }
                self.resolve_local(&identifier, expression);
            }
            Expression::Grouping(expression) => {
                self.resolve_expression(expression)?;
            }
            Expression::Assignment { identifier, value } => {
                self.resolve_expression(value)?;
                self.resolve_local(identifier, expression);
            }
            Expression::FunctionCall {
                function,
                arguments,
            } => {
                self.resolve_expression(function)?;
                for argument in arguments {
                    self.resolve_expression(argument)?;
                }
            }
        }
        Ok(())
    }

    fn resolve_function(
        &mut self,
        parameters: &Vec<String>,
        body: &Statement,
    ) -> Result<(), Error> {
        self.begin_scope();
        for parameter in parameters {
            self.declare(parameter); //TODO: this line makes no difference, right?
            self.define(parameter);
        }
        self.resolve_statement(body)?;
        self.end_scope();
        Ok(())
    }

    fn resolve_local(&mut self, identifier: &String, expression: &Expression) {
        if let Some(depth) = self.scopes.any_contains(identifier) {
            self.interpreter.resolve(expression.clone(), depth);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push();
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, identifier: &String) {
        self.scopes
            .insert(identifier.clone(), VariableStatus::Declared);
    }

    fn define(&mut self, identifier: &String) {
        self.scopes
            .insert(identifier.clone(), VariableStatus::Defined);
    }
}
