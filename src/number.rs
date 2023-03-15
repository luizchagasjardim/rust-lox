use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Number(f64);

impl Number {
    fn key(&self) -> u64 {
        self.0.to_bits()
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number(value)
    }
}

impl Hash for Number {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key().hash(state)
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Number {}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number(-self.0)
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Number(self.0 + rhs.0)
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Number(self.0 * rhs.0)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Number(self.0 / rhs.0)
    }
}
