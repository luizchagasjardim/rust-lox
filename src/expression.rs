pub enum Expression {
    Literal(Literal),
    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
}

enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub enum UnaryOperator {
    Negation,
    Minus,
}

pub enum BinaryOperator {
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

impl Expression {
    fn to_code(&self) -> String {
        match self {
            Expression::Literal(literal) => literal.to_code(),
            Expression::Unary {
                operator,
                expression,
            } => format!("{}{}", operator.to_code(), expression.to_code()),
            Expression::Binary {
                left,
                operator,
                right,
            } => format!(
                "{} {} {}",
                left.to_code(),
                operator.to_code(),
                right.to_code()
            ),
            Expression::Grouping(expression) => format!("({})", expression.to_code()),
        }
    }
}

impl Literal {
    fn to_code(&self) -> String {
        match self {
            Literal::Number(number) => number.to_string(),
            Literal::String(string) => format!("\"{}\"", string),
            Literal::True => "True".to_string(),
            Literal::False => "False".to_string(),
            Literal::Nil => "Nil".to_string(),
        }
    }
}

impl UnaryOperator {
    fn to_code(&self) -> String {
        match self {
            UnaryOperator::Negation => "!".to_string(),
            UnaryOperator::Minus => "-".to_string(),
        }
    }
}

impl BinaryOperator {
    fn to_code(&self) -> String {
        match self {
            BinaryOperator::Equality => "==".to_string(),
            BinaryOperator::Different => "!=".to_string(),
            BinaryOperator::Less => "<".to_string(),
            BinaryOperator::EqualOrLess => "<=".to_string(),
            BinaryOperator::Greater => ">".to_string(),
            BinaryOperator::EqualOrGreater => ">=".to_string(),
            BinaryOperator::Addition => "+".to_string(),
            BinaryOperator::Subtraction => "-".to_string(),
            BinaryOperator::Multiplication => "*".to_string(),
            BinaryOperator::Division => "/".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_literal_expression_to_code() {
        let expression = Expression::Literal(Literal::Number(3.14));
        assert_eq!(expression.to_code(), "3.14".to_string());
    }

    #[test]
    fn string_literal_expression_to_code() {
        let expression = Expression::Literal(Literal::String("Lorem Ipsum".to_string()));
        assert_eq!(expression.to_code(), "\"Lorem Ipsum\"".to_string());
    }

    #[test]
    fn true_expression_to_code() {
        let expression = Expression::Literal(Literal::True);
        assert_eq!(expression.to_code(), "True".to_string());
    }

    #[test]
    fn false_expression_to_code() {
        let expression = Expression::Literal(Literal::False);
        assert_eq!(expression.to_code(), "False".to_string());
    }

    #[test]
    fn nil_expression_to_code() {
        let expression = Expression::Literal(Literal::Nil);
        assert_eq!(expression.to_code(), "Nil".to_string());
    }

    #[test]
    fn negation_expression_to_code() {
        let literal = Expression::Literal(Literal::False);
        let expression = Expression::Unary {
            operator: UnaryOperator::Negation,
            expression: Box::new(literal),
        };
        assert_eq!(expression.to_code(), "!False".to_string());
    }

    #[test]
    fn minus_expression_to_code() {
        let literal = Expression::Literal(Literal::Number(4.2));
        let expression = Expression::Unary {
            operator: UnaryOperator::Minus,
            expression: Box::new(literal),
        };
        assert_eq!(expression.to_code(), "-4.2".to_string());
    }

    #[test]
    fn equality_expression_to_code() {
        let left = Expression::Literal(Literal::Number(6.66));
        let right = Expression::Literal(Literal::False);
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Equality,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "6.66 == False".to_string());
    }

    #[test]
    fn different_expression_to_code() {
        let left = Expression::Literal(Literal::Nil);
        let right = Expression::Literal(Literal::Nil);
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Different,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "Nil != Nil".to_string());
    }

    #[test]
    fn less_expression_to_code() {
        let left = Expression::Literal(Literal::Number(3.14));
        let right = Expression::Literal(Literal::Number(3.16));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Less,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "3.14 < 3.16".to_string());
    }

    #[test]
    fn equal_or_less_expression_to_code() {
        let left = Expression::Literal(Literal::Number(-3.16));
        let right = Expression::Literal(Literal::Number(-3.14));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::EqualOrLess,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "-3.16 <= -3.14".to_string());
    }

    #[test]
    fn greater_expression_to_code() {
        let left = Expression::Literal(Literal::String("Hello".to_string()));
        let right = Expression::Literal(Literal::String("World".to_string()));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Greater,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "\"Hello\" > \"World\"".to_string());
    }

    #[test]
    fn equal_or_greater_expression_to_code() {
        let left = Expression::Literal(Literal::String("Hello, world!".to_string()));
        let right = Expression::Literal(Literal::True);
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::EqualOrGreater,
            right: Box::new(right),
        };
        assert_eq!(
            expression.to_code(),
            "\"Hello, world!\" >= True".to_string()
        );
    }

    #[test]
    fn addition_expression_to_code() {
        let left = Expression::Literal(Literal::Number(1.2));
        let right = Expression::Literal(Literal::Number(3.4));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Addition,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "1.2 + 3.4".to_string());
    }

    #[test]
    fn subtraction_expression_to_code() {
        let left = Expression::Literal(Literal::Number(0.1));
        let right = Expression::Literal(Literal::Number(-0.1));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Subtraction,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "0.1 - -0.1".to_string()); //TODO: Do we really want this behaviour? No
    }

    #[test]
    fn multiplication_expression_to_code() {
        let left = Expression::Literal(Literal::False);
        let right = Expression::Literal(Literal::True);
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Multiplication,
            right: Box::new(right),
        };
        assert_eq!(expression.to_code(), "False * True".to_string());
    }

    #[test]
    fn division_expression_to_code() {
        let left = Expression::Literal(Literal::String(
            "No division by zero allowed!!!".to_string(),
        ));
        let right = Expression::Literal(Literal::Number(0.0));
        let expression = Expression::Binary {
            left: Box::new(left),
            operator: BinaryOperator::Division,
            right: Box::new(right),
        };
        assert_eq!(
            expression.to_code(),
            "\"No division by zero allowed!!!\" / 0".to_string()
        );
    }

    #[test]
    fn grouping_expression_to_code() {
        let literal = Expression::Literal(Literal::String(
            "The quick brown fox did WHAT!?".to_string(),
        ));
        let expression = Expression::Grouping(Box::new(literal));
        assert_eq!(
            expression.to_code(),
            "(\"The quick brown fox did WHAT!?\")".to_string()
        );
    }
}
