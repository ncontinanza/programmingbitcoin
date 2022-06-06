use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    num: i128,
    prime: i128,
}

#[derive(PartialEq, Debug)]
pub enum FieldElementError {
    FieldRangeError(String),
    FieldOrderError(String),
}

impl FieldElement {
    pub fn new(num: i128, prime: i128) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < 0 {
            return Err(FieldElementError::FieldRangeError(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn pow(self, exp: i128) -> FieldElement {
        let mut n = exp.rem_euclid(self.prime - 1) as u32;

        let mut num = 1;

        while n > 0 {
            num = (num * self.num).rem_euclid(self.prime);
            n -= 1;
        }

        FieldElement {
            num,
            prime: self.prime,
        }
    }

    pub fn prime(self) -> i128 {
        self.prime
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot add two numbers in different fields"
        );

        FieldElement {
            num: (self.num + rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot substract two numbers in different fields"
        );

        FieldElement {
            num: (self.num - rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        let zero = FieldElement {
            num: 0,
            prime: self.prime,
        };
        zero - self
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot multiply two numbers in different fields"
        );

        FieldElement {
            num: (self.num * rhs.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot divide two numbers in different fields"
        );

        assert_ne!(0, rhs.num, "Zero is not a valid divisor!");
        self * rhs.pow(self.prime - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(2, 31).unwrap();
        let c = FieldElement::new(15, 31).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(15, 31).unwrap();

        assert_eq!(a + b, FieldElement::new(17, 31).unwrap());

        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 31).unwrap();

        assert_eq!(a + b, FieldElement::new(7, 31).unwrap());
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(29, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();

        assert_eq!(a - b, FieldElement::new(25, 31).unwrap());

        let a = FieldElement::new(15, 31).unwrap();
        let b = FieldElement::new(30, 31).unwrap();

        assert_eq!(a - b, FieldElement::new(16, 31).unwrap());
    }

    #[test]
    fn test_neg() {
        let a = FieldElement::new(9, 19).unwrap();

        assert_eq!(-a, FieldElement::new(10, 19).unwrap());
        assert_eq!(-a + a, FieldElement::new(0, 19).unwrap());
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31).unwrap();
        let b = FieldElement::new(19, 31).unwrap();

        assert_eq!(a * b, FieldElement::new(22, 31).unwrap());
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(3), FieldElement::new(15, 31).unwrap());

        let a = FieldElement::new(5, 31).unwrap();
        let b = FieldElement::new(18, 31).unwrap();
        assert_eq!(a.pow(5) * b, FieldElement::new(16, 31).unwrap());
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();
        assert_eq!(a / b, FieldElement::new(4, 31).unwrap());

        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(-3), FieldElement::new(29, 31).unwrap());

        let a = FieldElement::new(4, 31).unwrap();
        let b = FieldElement::new(11, 31).unwrap();
        assert_eq!(a.pow(-4) * b, FieldElement::new(13, 31).unwrap());
    }
}
