use crate::evaluate::Evaluate;
use crate::expression::Expression;
use crate::object::{Error, Object};
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
        expression.evaluate(self)
    }

    pub fn execute(&mut self, statement: Statement) -> Result<Object, Error> {
        let statement = match statement {
            Statement::If {
                condition,
                then_statement,
                else_statement,
            } => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(*then_statement)?
                } else {
                    match else_statement {
                        None => Object::Nil,
                        Some(statement) => self.execute(*statement)?,
                    }
                }
            }
            Statement::Print(expression) => {
                println!("{}", self.evaluate(expression)?);
                Object::Nil
            }
            Statement::Expression(expression) => self.evaluate(expression)?,
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => {
                let value = if let Some(expression) = expression {
                    self.evaluate(expression)?
                } else {
                    Object::Nil
                };
                self.define(identifier, value.clone());
                value
            }
            Statement::While {
                expression,
                statement,
            } => {
                while self.evaluate(expression.clone())?.is_truthy() {
                    self.execute(*statement.clone())?;
                }
                Object::Nil
            }
            Statement::Block(statements) => {
                let mut block_env = self.new_child();
                for statement in statements {
                    block_env.execute(statement)?;
                }
                Object::Nil
            }
        };
        Ok(statement)
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
