use crate::expression::Expression;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Option<Expression>,
    },
    Block(Vec<Statement>),
}
