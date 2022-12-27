use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Object {
    pub fn unary_minus(self) -> Result<Object, String> {
        match self {
            Object::Number(number) => Ok(Object::Number(-number)),
            _ => Err("unary minus operation can only be called on number type".to_string()),
        }
    }
    pub fn is_truthy(self) -> bool {
        match self {
            Object::Number(_) => true,
            Object::String(_) => true,
            Object::Boolean(boolean) => boolean,
            Object::Null => false,
        }
    }
}