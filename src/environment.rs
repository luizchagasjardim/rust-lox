use crate::object::{Error, Object};
use std::collections::HashMap;

pub struct Environment<'a> {
    values: HashMap<String, Object>,
    enclosing: Option<&'a mut Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn child<'b>(env: &'a mut Environment<'b> ) -> Environment<'a> where 'b:'a {
        Environment::<'a> {
            values: HashMap::new(),
            enclosing: Some(env),
        }
    }
    // pai pai.child(pai)
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
