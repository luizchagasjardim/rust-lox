use crate::expression::Expression;

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Expression(Expression),
    If {
        condition: Expression,
        then_statement: Box<Statement>,
        else_statement: Option<Box<Statement>>,
    },
    Print(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Option<Expression>,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<String>,
        body: Box<Statement>,
    },
    While {
        expression: Expression,
        statement: Box<Statement>,
    },
    Block(Vec<Statement>),
}
