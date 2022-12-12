enum Expression {
    Literal(Literal),
    Unary(Box<Unary>),
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
}

enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

struct Grouping(Expression);

struct Unary(Expression);

struct Binary {
    left: Expression,
    operator: Operator,
    right: Expression,
}

enum Operator {
    Equality,
    Different,
    Less,
    EqualOrLess,
    Greater,
    EqualOrGreater,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
// | "+"  | "-"  | "*" | "/" ;