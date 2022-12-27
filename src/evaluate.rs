use crate::expression::*;
use crate::object::*;

pub trait Evaluate {
    fn evaluate(self) -> Result<Object, String>;
}

impl Evaluate for Expression {
    fn evaluate(self) -> Result<Object, String> {
        match self {
            Expression::Literal(literal) => literal.evaluate(),
            Expression::Unary { operator, expression } => {
                let expresssion_value = expression.evaluate()?;
                match operator {
                    UnaryOperator::Negation => Ok(Object::Boolean(!expresssion_value.is_truthy())),
                    UnaryOperator::Minus => expresssion_value.unary_minus(),
                }
            }
            Expression::Binary { left, operator, right } => {
                let left_value = left.evaluate()?;
                let right_value = right.evaluate()?;
                match operator {
                    BinaryOperator::Equality => Ok(Object::Boolean(left_value == right_value)),
                    BinaryOperator::Different => Ok(Object::Boolean(left_value != right_value)),
                    _ => todo!(),
                    // BinaryOperator::Less => left_value
                    // BinaryOperator::EqualOrLess => {}
                    // BinaryOperator::Greater => {}
                    // BinaryOperator::EqualOrGreater => {}
                    // BinaryOperator::Addition => {}
                    // BinaryOperator::Subtraction => {}
                    // BinaryOperator::Multiplication => {}
                    // BinaryOperator::Division => {}
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
            Literal::Nil => Object::Null,
        };
        Ok(object)
    }
}