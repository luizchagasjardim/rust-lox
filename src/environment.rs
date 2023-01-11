use std::cell::RefCell;
use crate::object::{Error, Object};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub struct Environment(Rc<RefCell<EnvironmentInner>>);

impl Deref for Environment {
    type Target = Rc<RefCell<EnvironmentInner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment(Rc::new(RefCell::new(EnvironmentInner::new())))
    }
    pub fn new_child(&self) -> Environment {
        EnvironmentInner::child(self)
    }
}

pub struct EnvironmentInner {
    values: HashMap<String, Object>,
    enclosing: Option<Environment>,
}

impl EnvironmentInner {
    pub fn new() -> EnvironmentInner {
        EnvironmentInner {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn child(env: &Environment) -> Environment {
        Environment(Rc::new(RefCell::new(EnvironmentInner {
            values: HashMap::new(),
            enclosing: Some(Environment(Rc::clone(&env.0))),
        })))
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
            return enclosing.borrow().get(name);
        };
        Ok(value.clone())
    }

    pub fn assign(&mut self, name: String, value: Object) -> Result<Object, Error> {
        if self.values.contains_key(&*name) {
            if let Some(val) = self.values.insert(name.clone(), value.clone()) {
                if let Some(enclosing) = &mut self.enclosing {
                    return enclosing.borrow_mut().assign(name, value);
                }
                return Ok(val);
            };
        }
        Err(Error::UndefinedVariable)
    }
}
