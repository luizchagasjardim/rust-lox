use crate::expression::*;
use crate::object::*;
use crate::statement::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment(Rc<RefCell<EnvironmentInner>>);

impl Environment {
    pub fn new() -> Environment {
        Environment(Rc::new(RefCell::new(EnvironmentInner::new())))
    }
    pub fn new_child(&self) -> Environment {
        Environment(Rc::new(RefCell::new(EnvironmentInner::new_child(self))))
    }
    pub fn end(&mut self) -> Option<Environment> {
        self.0.borrow_mut().enclosing.clone()
    }
    pub fn define(&mut self, name: String, value: Object) {
        (*self.0).borrow_mut().define(name, value)
    }
    pub fn get(&self, name: &String) -> Result<Object, Error> {
        self.0.borrow().get(name)
    }
    pub fn assign(&mut self, name: String, value: Object) -> Result<Object, Error> {
        (*self.0).borrow_mut().assign(name, value)
    }

    pub fn evaluate(&mut self, expression: Expression) -> Result<Object, Error> {
        match expression {
            Expression::Literal(literal) => {
                let object = match literal {
                    Literal::Number(number) => Object::Number(number),
                    Literal::String(string) => Object::String(string),
                    Literal::True => Object::Boolean(true),
                    Literal::False => Object::Boolean(false),
                    Literal::Nil => Object::Nil,
                };
                Ok(object)
            }
            Expression::Unary {
                operator,
                expression,
            } => {
                let expresssion_value = self.evaluate(*expression)?;
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
                let left_value = self.evaluate(*left)?;
                let right_value = self.evaluate(*right)?;
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
                    BinaryOperator::Or => Ok(if left_value.is_truthy() {
                        left_value
                    } else {
                        right_value
                    }),
                    BinaryOperator::And => Ok(if left_value.is_truthy() {
                        right_value
                    } else {
                        left_value
                    }),
                }
            }
            Expression::Variable(string) => self.get(&string),
            Expression::Assignment { identifier, value } => {
                let value = self.evaluate(*value)?;
                self.assign(identifier, value)
            }
            Expression::Grouping(expression) => self.evaluate(*expression),
            Expression::FunctionCall {
                function,
                arguments,
            } => {
                let function_object = self.evaluate(*function)?;
                let Object::Function(mut function) = function_object else {
                    return Err(Error::AttemptedToCallUncallableExpression{ called: function_object });
                };
                if arguments.len() != function.arity() {
                    return Err(Error::WrongNumberOfArguments {
                        expected: function.arity(),
                        actual: arguments.len(),
                    });
                }
                let mut arguments = arguments
                    .into_iter()
                    .map(|arg| self.evaluate(arg))
                    .collect::<Result<Vec<Object>, Error>>()?;
                self.call_function(function, arguments)
            }
        }
    }

    fn call_function(
        &mut self,
        function: Rc<dyn Function>,
        arguments: Vec<Object>,
    ) -> Result<Object, Error> {
        todo!();
    }
}

struct EnvironmentInner {
    values: HashMap<String, Object>,
    enclosing: Option<Environment>,
}

impl EnvironmentInner {
    fn new() -> EnvironmentInner {
        EnvironmentInner {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    fn new_child(enclosing: &Environment) -> EnvironmentInner {
        EnvironmentInner {
            values: HashMap::new(),
            enclosing: Some(enclosing.clone()),
        }
    }

    fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &String) -> Result<Object, Error> {
        let Some(value) = self.values.get(name) else {
            let Some(enclosing) = &self.enclosing else {
                return Err(Error::UndefinedVariable);
            };
            return enclosing.get(name);
        };
        Ok(value.clone())
    }

    fn variable_was_declared_in_this_scope(&self, name: &String) -> bool {
        self.values.contains_key(name)
    }

    fn assign(&mut self, name: String, value: Object) -> Result<Object, Error> {
        if self.variable_was_declared_in_this_scope(&name) {
            let Some(val) = self.values.insert(name, value) else {
                unreachable!("We already checked that the key is in the map")
            };
            Ok(val)
        } else {
            let Some(enclosing) = &mut self.enclosing else {
                return Err(Error::UndefinedVariable);
            };
            enclosing.assign(name, value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;
    #[test]
    fn environment_define() {
        let mut env = Environment::new();
        env.define("x".to_string(), Object::Number(20.0));
        let result = env.get(&"x".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(20.0));
    }

    #[test]
    fn environment_get() {
        let mut env = Environment::new();
        env.define("x".to_string(), Object::Number(20.0));
        let result = env.get(&"x".to_string());
        let result_err = env.get(&"e".to_string());
        assert!(result.is_ok());
        assert!(result_err.is_err());
        assert_eq!(result.unwrap(), Object::Number(20.0));
        assert_matches!(result_err.unwrap_err(), Error::UndefinedVariable);
    }

    #[test]
    fn environment_assign() {
        let mut env = Environment::new();
        env.define("x".to_string(), Object::Number(20.0));

        let result = env.assign("x".to_string(), Object::Number(30.0));
        let result_err = env.assign("e".to_string(), Object::Number(20.0));
        assert!(result.is_ok());
        assert!(result_err.is_err());
        assert_eq!(result.unwrap(), Object::Number(20.0));
        assert_matches!(result_err.unwrap_err(), Error::UndefinedVariable);
    }

    #[test]
    fn environment_jested() {
        let mut env = Environment::new();
        env.define("x".to_string(), Object::Number(20.0));
        let mut env_nested = env.new_child();
        env_nested.define("y".to_string(), Object::Number(10.0));
        let result = env.get(&"x".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(20.0));
        let result = env_nested.get(&"y".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Object::Number(10.0));
    }
}
