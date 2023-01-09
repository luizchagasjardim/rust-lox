use crate::expression::Expression;

pub enum Statement {
    Expression(Expression),
    Print(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Option<Expression>,
    },
    Block(Vec<Statement>),
}
