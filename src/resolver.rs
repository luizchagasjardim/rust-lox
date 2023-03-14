use crate::expression::Expression;
use crate::interpreter::Interpreter;
use crate::statement::Statement;
use std::collections::HashMap;

enum VariableStatus {
    Declared,
    Defined,
}

struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, VariableStatus>>,
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expression(_) => todo!(),
            Statement::If { .. } => todo!(),
            Statement::Print(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                self.declare(identifier.clone());
                if let Some(initializer) = expression {
                    self.resolve_expression(initializer);
                }
                self.define(identifier);
            }
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::While { .. } => todo!(),
            Statement::Block(statements) => {
                self.begin_scope();
                for statement in statements {
                    self.resolve_statement(statement);
                }
                self.end_scope();
            }
        }
    }

    fn resolve_expression(&mut self, expression: Expression) {
        match expression {
            Expression::Literal(_) => todo!(),
            Expression::Unary { .. } => todo!(),
            Expression::Binary { .. } => todo!(),
            Expression::Variable(_) => todo!(),
            Expression::Grouping(_) => todo!(),
            Expression::Assignment { .. } => todo!(),
            Expression::FunctionCall { .. } => todo!(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, identifier: String) {
        let Some(scope) = self.scopes.last_mut() else {
            return;
        };
        scope.insert(identifier, VariableStatus::Declared);
    }

    fn define(&mut self, identifier: String) {
        let Some(scope) = self.scopes.last_mut() else {
            return;
        };
        scope.insert(identifier, VariableStatus::Defined);
    }
}
