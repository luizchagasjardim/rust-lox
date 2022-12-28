use crate::expression::*;
use crate::object::*;

pub trait Evaluate {
    fn evaluate(self) -> Result<Object, String>;
}

impl Evaluate for Expression {
    fn evaluate(self) -> Result<Object, String> {
        match self {
            Expression::Literal(literal) => literal.evaluate(),
            Expression::Unary {
                operator,
                expression,
            } => {
                let expresssion_value = expression.evaluate()?;
                match operator {
                    UnaryOperator::Negation => Ok(Object::Boolean(!expresssion_value.is_truthy())),
                    UnaryOperator::Minus => expresssion_value.unary_minus(),
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = left.evaluate()?;
                let right_value = right.evaluate()?;
                match operator {
                    BinaryOperator::Equality => Ok(Object::Boolean(left_value == right_value)),
                    BinaryOperator::Different => Ok(Object::Boolean(left_value != right_value)),
                    BinaryOperator::Less => Ok(Object::Boolean(left_value < right_value)),
                    BinaryOperator::EqualOrLess => Ok(Object::Boolean(left_value <= right_value)),
                    BinaryOperator::Greater => Ok(Object::Boolean(left_value > right_value)),
                    BinaryOperator::EqualOrGreater => {
                        Ok(Object::Boolean(left_value >= right_value))
                    }
                    BinaryOperator::Addition => left_value + right_value,
                    BinaryOperator::Subtraction => left_value - right_value,
                    BinaryOperator::Multiplication => left_value * right_value,
                    BinaryOperator::Division => left_value / right_value,
                }
            }
            Expression::Grouping(expression) => expression.evaluate(),
        }
    }
}

impl Evaluate for Literal {
    fn evaluate(self) -> Result<Object, String> {
        let object = match self {
            Literal::Number(number) => Object::Number(number),
            Literal::String(string) => Object::String(string),
            Literal::True => Object::Boolean(true),
            Literal::False => Object::Boolean(false),
            Literal::Nil => Object::Nil,
        };
        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_literal() {
        let test_values = vec![
            (Literal::Number(123.4), Object::Number(123.4)),
            (
                Literal::String("hello".to_string()),
                Object::String("hello".to_string()),
            ),
            (Literal::True, Object::Boolean(true)),
            (Literal::False, Object::Boolean(false)),
            (Literal::Nil, Object::Nil),
        ];

        for (lit, obj) in test_values {
            let result = lit.evaluate();
            assert_eq!(result, Ok(obj));
        }
    }

    #[test]
    fn evaluate_unary() {
        let test_values = vec![
            (
                Expression::Unary {
                    operator: UnaryOperator::Negation,
                    expression: Box::new(Expression::Literal(Literal::True)),
                },
                Object::Boolean(false),
            ),
            (
                Expression::Unary {
                    operator: UnaryOperator::Negation,
                    expression: Box::new(Expression::Literal(Literal::False)),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Unary {
                    operator: UnaryOperator::Minus,
                    expression: Box::new(Expression::Literal(Literal::Number(123.4))),
                },
                Object::Number(-123.4),
            ),
        ];

        for (input, obj) in test_values {
            let result = input.evaluate();
            assert_eq!(result, Ok(obj));
        }
    }

    #[test]
    fn evaluate_binary() {
        let test_values = vec![
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(1.0))),
                    operator: BinaryOperator::Addition,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Number(3.0),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(2.0))),
                    operator: BinaryOperator::Subtraction,
                    right: Box::new(Expression::Literal(Literal::Number(1.0))),
                },
                Object::Number(1.0),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(2.0))),
                    operator: BinaryOperator::Multiplication,
                    right: Box::new(Expression::Literal(Literal::Number(3.0))),
                },
                Object::Number(6.0),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(4.0))),
                    operator: BinaryOperator::Division,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Number(2.0),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(4.0))),
                    operator: BinaryOperator::Greater,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(1.0))),
                    operator: BinaryOperator::Less,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(1.0))),
                    operator: BinaryOperator::EqualOrLess,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(2.0))),
                    operator: BinaryOperator::EqualOrGreater,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(2.0))),
                    operator: BinaryOperator::Equality,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
            (
                Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Number(3.0))),
                    operator: BinaryOperator::Different,
                    right: Box::new(Expression::Literal(Literal::Number(2.0))),
                },
                Object::Boolean(true),
            ),
        ];
        for (input, obj) in test_values {
            let result = input.evaluate();
            assert_eq!(result, Ok(obj));
        }
    }
}
