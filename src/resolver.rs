use crate::expression::Expression;
use crate::interpreter::Interpreter;
use crate::statement::Statement;

struct Resolver {
    interpreter: Interpreter,
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Resolver { interpreter }
    }

    fn resolve_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Expression(_) => todo!(),
            Statement::If { .. } => todo!(),
            Statement::Print(_) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::VariableDeclaration { .. } => todo!(),
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::While { .. } => todo!(),
            Statement::Block(_) => todo!(),
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
}
