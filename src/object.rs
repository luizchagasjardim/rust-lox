use crate::interpreter::Interpreter;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub trait Function: Debug {
    fn arity(&self) -> usize;
    fn call(&mut self, interpreter: &Interpreter, arguments: Vec<Object>) -> Object;
}

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(Rc<dyn Function>),
    Nil,
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(num), Object::Number(other_num)) => num == other_num,
            (Object::String(str), Object::String(other_str)) => str == other_str,
            (Object::Boolean(bool), Object::Boolean(other_bool)) => bool == other_bool,
            (Object::Nil, Object::Nil) => true,
            _ => todo!(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Object {
    pub fn unary_minus(self) -> Result<Object, Error> {
        let number = self.to_number_value()?;
        Ok(Object::Number(-number))
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Number(_) => true,
            Object::String(_) => true,
            Object::Boolean(boolean) => *boolean,
            Object::Function(_) => todo!(),
            Object::Nil => false,
        }
    }
    pub fn to_number_value(&self) -> Result<f64, Error> {
        match self {
            Object::Number(number) => Ok(*number),
            _ => Err(Error::ExpectedNumber {
                actual: self.clone(),
            }),
        }
    }
    pub fn to_string_value(self) -> Result<String, Error> {
        match self {
            Object::String(string) => Ok(string),
            _ => Err(Error::ExpectedString { actual: self }),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Object::Number(number) => number.partial_cmp(&other.to_number_value().ok()?),
            _ => None,
        }
    }
}

impl std::ops::Add for Object {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => Ok(Object::Number(number + rhs.to_number_value()?)),
            Object::String(string) => Ok(Object::String(string + &rhs.to_string_value()?)),
            _ => Err(Error::ExpectedNumberOrString { actual: self }),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.unary_minus()?
    }
}

impl std::ops::Mul for Object {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => Ok(Object::Number(number * rhs.to_number_value()?)),
            _ => Err(Error::ExpectedNumber { actual: self }),
        }
    }
}

impl std::ops::Div for Object {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => {
                let divisor = rhs.to_number_value()?;
                if divisor == 0.0 {
                    return Err(Error::DivisionByZero);
                }
                Ok(Object::Number(number / divisor))
            }
            _ => Err(Error::ExpectedNumber { actual: self }),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    AttemptedToCallUncallableExpression { called: Object },
    ExpectedNumber { actual: Object },
    ExpectedString { actual: Object },
    ExpectedNumberOrString { actual: Object },
    UndefinedVariable,
    DivisionByZero,
    WrongNumberOfArguments { expected: usize, actual: usize },
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AttemptedToCallUncallableExpression { called } => {
                write!(
                    formatter,
                    "Attempted to call uncallable expresion {}.",
                    called
                )
            }
            Error::ExpectedNumber { actual } => {
                write!(formatter, "Expected number, found {}.", actual)
            }
            Error::ExpectedString { actual } => {
                write!(formatter, "Expected string, found {}.", actual)
            }
            Error::ExpectedNumberOrString { actual } => {
                write!(formatter, "Expected number or string, found {}.", actual)
            }
            Error::UndefinedVariable => write!(formatter, "UndefinedVariable."),
            Error::DivisionByZero => write!(formatter, "Division by zero."),
            Error::WrongNumberOfArguments { expected, actual } => {
                write!(formatter, "Wrong number of arguments. Function expects {} arguments, but got called with {} arguments", expected, actual)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unary_minus() {
        assert_eq!(
            Object::Number(1.0).unary_minus().unwrap(),
            Object::Number(-1.0)
        );
    }

    #[test]
    fn unary_minus_invalid_input() {
        assert!(Object::String("hello".to_string()).unary_minus().is_err());
        assert!(Object::Boolean(false).unary_minus().is_err());
        assert!(Object::Nil.unary_minus().is_err());
    }

    #[test]
    fn is_truthy_number() {
        assert!(Object::Number(0.0).is_truthy());
    }

    #[test]
    fn is_truthy_string() {
        assert!(Object::String("".to_string()).is_truthy());
    }

    #[test]
    fn is_truthy_true() {
        assert!(Object::Boolean(true).is_truthy());
    }

    #[test]
    fn is_truthy_false() {
        assert_eq!(Object::Boolean(false).is_truthy(), false);
    }

    #[test]
    fn is_truthy_nil() {
        assert_eq!(Object::Nil.is_truthy(), false);
    }

    #[test]
    fn to_number_value_number() {
        assert!(Object::Number(1.0).to_number_value().is_ok())
    }
    #[test]
    fn to_number_value_string() {
        assert!(Object::String("a".to_string()).to_number_value().is_err())
    }

    #[test]
    fn to_number_value_boolean() {
        assert!(Object::Boolean(true).to_number_value().is_err())
    }

    #[test]
    fn to_number_value_nil() {
        assert!(Object::Nil.to_number_value().is_err())
    }

    #[test]
    fn to_string_value_number() {
        assert!(Object::Number(1.0).to_string_value().is_err())
    }
    #[test]
    fn to_string_value_string() {
        assert!(Object::String("a".to_string()).to_string_value().is_ok())
    }

    #[test]
    fn to_string_value_boolean() {
        assert!(Object::Boolean(true).to_string_value().is_err())
    }

    #[test]
    fn to_string_value_nil() {
        assert!(Object::Nil.to_string_value().is_err())
    }

    #[test]
    fn greater() {
        assert!(Object::Number(1.0) > Object::Number(0.0))
    }

    #[test]
    fn greater_nil() {
        assert!(!(Object::Nil > Object::Nil))
    }

    #[test]
    fn add_numbers() {
        let result = Object::Number(1.0) + Object::Number(2.0);
        assert_eq!(result.unwrap(), Object::Number(3.0))
    }

    #[test]
    fn add_strings() {
        let result = Object::String("hello".to_string()) + Object::String(" world!".to_string());
        assert_eq!(result.unwrap(), Object::String("hello world!".to_string()))
    }

    #[test]
    fn add_bool() {
        assert!((Object::Boolean(false) + (Object::Boolean(false))).is_err())
    }

    #[test]
    fn add_nil() {
        assert!((Object::Nil + (Object::Nil)).is_err())
    }

    #[test]
    fn subtract_numbers() {
        let result = Object::Number(1.0) - Object::Number(2.0);
        assert_eq!(result.unwrap(), Object::Number(-1.0))
    }

    #[test]
    fn subtract_strings() {
        assert!(
            (Object::String("hello".to_string()) - Object::String(" world!".to_string())).is_err()
        )
    }

    #[test]
    fn subtract_bool() {
        assert!((Object::Boolean(true) - Object::Boolean(true)).is_err())
    }

    #[test]
    fn subtract_nil() {
        assert!((Object::Nil - Object::Nil).is_err())
    }

    #[test]
    fn multiply_number() {
        let result = Object::Number(2.0) * Object::Number(2.0);
        assert_eq!(result.unwrap(), Object::Number(4.0))
    }
    #[test]
    fn multiply_string() {
        assert!((Object::String("a".to_string()) * Object::String("a".to_string())).is_err())
    }

    #[test]
    fn multiply_boolean() {
        assert!((Object::Boolean(true) * Object::Boolean(false)).is_err())
    }

    #[test]
    fn multiply_nil() {
        assert!((Object::Nil * Object::Nil).is_err())
    }

    #[test]
    fn divide_number() {
        let result = Object::Number(2.0) / Object::Number(2.0);
        assert_eq!(result.unwrap(), Object::Number(1.0))
    }

    #[test]
    fn divide_number_by_0() {
        assert!((Object::Number(2.0) / Object::Number(0.0)).is_err())
    }

    #[test]
    fn divide_string() {
        assert!((Object::String("a".to_string()) / Object::String("a".to_string())).is_err())
    }

    #[test]
    fn divide_boolean() {
        assert!((Object::Boolean(true) / Object::Boolean(false)).is_err())
    }

    #[test]
    fn divide_nil() {
        assert!((Object::Nil / Object::Nil).is_err())
    }
}
