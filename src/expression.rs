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

enum UnaryOperator {
    Negation,
    Minus,
}

struct Unary {
    operator: UnaryOperator,
    expression: Expression,
}

struct Binary {
    left: Expression,
    operator: BinaryOperator,
    right: Expression,
}

enum BinaryOperator {
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

trait ExpressionTrait {
    fn to_code(&self) -> String;
}

impl ExpressionTrait for Expression {
    fn to_code(&self) -> String {
        match self {
            Expression::Literal(literal) => literal.to_code(),
            Expression::Unary(unary) => unary.to_code(),
            Expression::Binary(binary) => binary.to_code(),
            Expression::Grouping(grouping) => grouping.to_code(),
        }
    }
}

impl ExpressionTrait for Literal {
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

impl ExpressionTrait for Unary {
    fn to_code(&self) -> String {
        format!("{}{}", self.operator.to_code(), self.expression.to_code())
    }
}

impl ExpressionTrait for Binary {
    fn to_code(&self) -> String {
        format!(
            "{} {} {}",
            self.left.to_code(),
            self.operator.to_code(),
            self.right.to_code()
        )
    }
}

impl ExpressionTrait for Grouping {
    fn to_code(&self) -> String {
        format!("({})", self.0.to_code())
    }
}

impl ExpressionTrait for UnaryOperator {
    fn to_code(&self) -> String {
        match self {
            UnaryOperator::Negation => "!".to_string(),
            UnaryOperator::Minus => "-".to_string(),
        }
    }
}

impl ExpressionTrait for BinaryOperator {
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
        let unary = Unary {
            operator: UnaryOperator::Negation,
            expression: literal,
        };
        let expression = Expression::Unary(Box::<Unary>::new(unary));
        assert_eq!(expression.to_code(), "!False".to_string());
    }

    #[test]
    fn minus_expression_to_code() {
        let literal = Expression::Literal(Literal::Number(4.2));
        let unary = Unary {
            operator: UnaryOperator::Minus,
            expression: literal,
        };
        let expression = Expression::Unary(Box::<Unary>::new(unary));
        assert_eq!(expression.to_code(), "-4.2".to_string());
    }

    #[test]
    fn grouping_expression_to_code() {
        let literal = Expression::Literal(Literal::String(
            "The quick brown fox did WHAT!?".to_string(),
        ));
        let grouping = Grouping(literal);
        let expression = Expression::Grouping(Box::<Grouping>::new(grouping));
        assert_eq!(
            expression.to_code(),
            "(\"The quick brown fox did WHAT!?\")".to_string()
        );
    }
}
