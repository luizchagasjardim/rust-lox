/* Grammar
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
*/

struct Expression(Equality);

struct Equality {
    left_operand: Comparison,
    right: Vec<EqualityRight>,
}

enum EqualityOperator {
    Equal,
    Different,
}

struct EqualityRight {
    operator: EqualityOperator,
    right_operand: Comparison,
}

struct Comparison {
    left_operand: Term,
    right: Vec<ComparisonRight>,
}

enum ComparisonOperator {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

struct ComparisonRight {
    operator: ComparisonOperator,
    right_operand: Term,
}

struct Term {
    left_operand: Factor,
    right: Vec<TermRight>,
}

enum TermOperator {
    Addition,
    Subtraction,
}

struct TermRight {
    operator: TermOperator,
    right_operand: Factor,
}

struct Factor {
    left_operand: Unary,
    right: Vec<FactorRight>,
}

enum FactorOperator {
    Division,
    Multiplication,
}

struct FactorRight {
    operator: FactorOperator,
    right_operand: Unary,
}

struct Unary {
    operator: UnaryOperator,
    operand: Box<UnaryOrPrimary>,
}

enum UnaryOperator {
    Bang,
    Minus,
}

enum UnaryOrPrimary {
    Unary(Unary),
    Primary(Primary),
}

enum Primary {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Grouping(Box<Expression>),
}

