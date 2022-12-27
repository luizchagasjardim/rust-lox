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
