use crate::object::{Error, Object};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn child(env: Environment) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::from(env)),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Result<Object, Error> {
        let Some(value) = self.values.get(name) else {
            let Some(enclosing) = &self.enclosing else {
                return Err(Error::UndefinedVariable);
            };
            return enclosing.get(name);
        };
        Ok(value.clone())
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<Object, Error> {
        if self.values.contains_key(&*name) {
            if let Some(val) = self.values.insert(name.clone(), value.clone()) {
                if let Some(enclosing) = &mut self.enclosing {
                    return enclosing.assign(name, value);
                }
                return Ok(val);
            };
        }
        Err(Error::UndefinedVariable)
    }
}
