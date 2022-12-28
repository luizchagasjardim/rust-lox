use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
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
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Number(_) => true,
            Object::String(_) => true,
            Object::Boolean(boolean) => *boolean,
            Object::Nil => false,
        }
    }
    pub fn to_number_value(&self) -> Result<f64, String> {
        match self {
            Object::Number(number) => Ok(*number),
            Object::String(_) => Err("Cannot implicitly convert string to number.".to_string()),
            Object::Boolean(_) => Err("Cannot implicitly convert boolean to number.".to_string()),
            Object::Nil => Err("Cannot implicitly convert nil to number.".to_string()),
        }
    }
    pub fn to_string_value(self) -> Result<String, String> {
        match self {
            Object::Number(_) => Err("Cannot implicitly convert number to string.".to_string()),
            Object::String(string) => Ok(string),
            Object::Boolean(_) => Err("Cannot implicitly convert boolean to string.".to_string()),
            Object::Nil => Err("Cannot implicitly convert nil to string.".to_string()),
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
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => Ok(Object::Number(number + rhs.to_number_value()?)),
            Object::String(string) => Ok(Object::String(string + &rhs.to_string_value()?)),
            Object::Boolean(_) => Err("Cannot add boolean.".to_string()),
            Object::Nil => Err("Cannot add nil.".to_string()),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Result<Self, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.unary_minus()?
    }
}

impl std::ops::Mul for Object {
    type Output = Result<Self, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => Ok(Object::Number(number * rhs.to_number_value()?)),
            Object::String(_) => Err("Cannot multiply string.".to_string()),
            Object::Boolean(_) => Err("Cannot multiply boolean.".to_string()),
            Object::Nil => Err("Cannot multiply nil.".to_string()),
        }
    }
}

impl std::ops::Div for Object {
    type Output = Result<Self, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Object::Number(number) => {
                let divisor = rhs.to_number_value()?;
                if divisor == 0.0 {
                    return Err("Cannot divide by zero".to_string());
                }
                Ok(Object::Number(number / divisor))
            }
            Object::String(_) => Err("Cannot divide string.".to_string()),
            Object::Boolean(_) => Err("Cannot divide boolean.".to_string()),
            Object::Nil => Err("Cannot divide nil.".to_string()),
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
        assert_eq!(
            Object::Number(1.0) + Object::Number(2.0),
            Ok(Object::Number(3.0))
        )
    }

    #[test]
    fn add_strings() {
        assert_eq!(
            Object::String("hello".to_string()) + Object::String(" world!".to_string()),
            Ok(Object::String("hello world!".to_string()))
        )
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
        assert_eq!(
            Object::Number(1.0) - Object::Number(2.0),
            Ok(Object::Number(-1.0))
        )
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
        assert_eq!(
            Object::Number(2.0) * Object::Number(2.0),
            Ok(Object::Number(4.0))
        )
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
        assert_eq!(
            Object::Number(2.0) / Object::Number(2.0),
            Ok(Object::Number(1.0))
        )
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
