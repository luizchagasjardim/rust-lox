use crate::environment::Environment;
use crate::expression::*;
use crate::object::*;
use crate::statement::Statement;

pub trait Evaluate {
    fn evaluate(self, environment: &mut Environment) -> Result<Object, Error>;
}

impl Evaluate for Expression {
    fn evaluate(self, environment: &mut Environment) -> Result<Object, Error> {
        match self {
            Expression::Literal(literal) => literal.evaluate(environment),
            Expression::Unary {
                operator,
                expression,
            } => {
                let expresssion_value = expression.evaluate(environment)?;
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
                let left_value = left.evaluate(environment)?;
                let right_value = right.evaluate(environment)?;
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
            Expression::Variable(string) => environment.get(&string),
            Expression::Assignment { identifier, value } => {
                let value = value.evaluate(environment)?;
                environment.assign(identifier, value)
            },
            Expression::Grouping(expression) => expression.evaluate(environment),
        }
    }
}

impl Evaluate for Literal {
    fn evaluate(self, _: &mut Environment) -> Result<Object, Error> {
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

impl Evaluate for Statement {
    fn evaluate(self, environment: &mut Environment) -> Result<Object, Error> {
        let statement = match self {
            Statement::Print(expression) => {
                println!("{}", expression.evaluate(environment)?);
                Object::Nil
            }
            Statement::Expression(expression) => expression.evaluate(environment)?,
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                let value = if let Some(expression) = expression {
                    expression.evaluate(environment)?
                } else {
                    Object::Nil
                };
                environment.define(identifier, value.clone());
                value
            }
            Statement::Block(statements) => {
                let mut block_env = environment.new_child();
                for statement in statements {
                    statement.evaluate(&mut block_env)?;
                }
                Object::Nil
            }
        };
        Ok(statement)
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
                Literal::String(
                    "hello
            }"
                    .to_string(),
                ),
                Object::String("hello".to_string()),
            ),
            (Literal::True, Object::Boolean(true)),
            (Literal::False, Object::Boolean(false)),
            (Literal::Nil, Object::Nil),
        ];

        for (lit, obj) in test_values {
            let result = lit.evaluate().unwrap();
            assert_eq!(result, obj);
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
            let result = input.evaluate().unwrap();
            assert_eq!(result, obj);
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
            let result = input.evaluate().unwrap();
            assert_eq!(result, obj);
        }
    }
}
