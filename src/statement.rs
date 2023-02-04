use crate::expression::Expression;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Expression(Expression),
    If {
        condition: Expression,
        then_statement: Box<Statement>,
        else_statement: Box<Statement>,
    },
    Print(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Option<Expression>,
    },
    Block(Vec<Statement>),
}
