use crate::object::{Error, Object};
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
