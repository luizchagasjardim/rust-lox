use crate::object::{Error, Object};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Result<Object, Error> {
        let Some(value) = self.values.get(name) else {
            return Err(Error::UndefinedVariable);
        };
        Ok(value.clone())
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<Object, Error> {
        if self.values.contains_key(&*name) {
            if let Some(val) = self.values.insert(name, value) {
                return Ok(val);
            };
        }
        Err(Error::UndefinedVariable)
    }
}
