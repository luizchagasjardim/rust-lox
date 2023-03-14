use crate::expression::Expression;
use crate::interpreter::Interpreter;
use crate::map_stack::MapStack;
use crate::result::Error;
use crate::statement::Statement;

#[derive(PartialEq)]
enum VariableStatus {
    Declared,
    Defined,
}

struct Resolver {
    interpreter: Interpreter,
    scopes: MapStack<String, VariableStatus>,
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: MapStack::new(),
        }
    }

    fn resolve_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression(_) => todo!(),
            Statement::If { .. } => todo!(),
            Statement::Print(_) => todo!(),
            Statement::Return(_) => todo!(),
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
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::While { .. } => todo!(),
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
            Expression::Literal(_) => todo!(),
            Expression::Unary { .. } => todo!(),
            Expression::Binary { .. } => todo!(),
            Expression::Variable(identifier) => {
                if self.scopes.get_in_top(identifier) == Some(&VariableStatus::Declared) {
                    return Err(todo!());
                }
                self.resolve_local(&identifier, expression);
            }
            Expression::Grouping(_) => todo!(),
            Expression::Assignment { identifier, value } => {
                self.resolve_expression(value)?;
                self.resolve_local(identifier, expression);
            }
            Expression::FunctionCall { .. } => todo!(),
        }
        Ok(())
    }

    fn resolve_local(&mut self, identifier: &String, expression: &Expression) {
        if let Some(depth) = self.scopes.any_contains(identifier) {
            self.interpreter.resolve(expression, identifier, depth);
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
