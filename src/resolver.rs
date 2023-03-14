use crate::expression::Expression;
use crate::interpreter::Interpreter;
use crate::map_stack::MapStack;
use crate::statement::Statement;

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
        self.scopes.push();
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, identifier: String) {
        self.scopes.insert(identifier, VariableStatus::Declared);
    }

    fn define(&mut self, identifier: String) {
        self.scopes.insert(identifier, VariableStatus::Defined);
    }
}
