use crate::expression::Expression;

#[derive(Clone, PartialEq, Debug)]
pub struct FunctionDeclaration {
    pub identifier: String,
    pub parameters: Vec<String>,
    pub body: Box<Statement>,
}

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
    FunctionDeclaration(FunctionDeclaration),
    While {
        expression: Expression,
        statement: Box<Statement>,
    },
    Block(Vec<Statement>),
}
