use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_bigint::BigInt;

#[derive(PartialEq, Debug, Clone)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

#[derive(PartialEq, Debug)]
pub enum FieldElementError {
    FieldRangeError(String),
    FieldOrderError(String),
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < BigInt::from(0) {
            return Err(FieldElementError::FieldRangeError(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn pow(&self, exp: &BigInt) -> FieldElement {
        /*let mut n = exp.rem_euclid(self.prime - BigInt::from(1)) as u32;
        
        let mut num = BigInt::from(1i32);

        while n > 0 {
            num = (num * self.num).rem_euclid(self.prime);
            n -= 1;
        }

        FieldElement {
            num,
            prime: self.prime,
        }
    } */
    let mut n = exp % (self.prime - BigInt::from(1i32));
    let num = self.num.modpow(&n, &self.prime);
    return FieldElement { num, prime: self.prime }

}


    pub fn prime(self) -> BigInt {
        self.prime
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

// TODO: Implement AddAssign, SubAssign, MulAssign and DivAssign
impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot add two numbers in different fields"
        );

        FieldElement {
            num: (self.num + rhs.num) % (self.prime),
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
            num: (self.num - rhs.num) % (self.prime),
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        let zero = FieldElement {
            num: BigInt::from(0),
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
            num: (self.num * rhs.num) % (self.prime),
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

        assert_ne!(BigInt::from(0i32), rhs.num, "Zero is not a valid divisor!");
        self * rhs.pow(&(self.prime - BigInt::from(2)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: make these tests really unit tests
    #[test]
    fn test_ne() {
        let a = FieldElement::new(BigInt::from(2i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(2i32), BigInt::from(31i32)).unwrap();
        let c = FieldElement::new(BigInt::from(15i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(BigInt::from(2i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(15i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a + b, FieldElement::new(BigInt::from(17i32), BigInt::from(31i32)).unwrap());

        let a = FieldElement::new(BigInt::from(17i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(21i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a + b, FieldElement::new(BigInt::from(7i32), BigInt::from(31i32)).unwrap());
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(BigInt::from(29i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(4i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a - b, FieldElement::new(BigInt::from(25i32), BigInt::from(31i32)).unwrap());

        let a = FieldElement::new(BigInt::from(15i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(30i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a - b, FieldElement::new(BigInt::from(16i32), BigInt::from(31i32)).unwrap());
    }

    #[test]
    fn test_neg() {
        let a = FieldElement::new(BigInt::from(9i32), BigInt::from(19i32)).unwrap();

        assert_eq!(-a, FieldElement::new(BigInt::from(10i32), BigInt::from(19i32)).unwrap());
        assert_eq!(-a + a, FieldElement::new(BigInt::from(0i32), BigInt::from(19i32)).unwrap());
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(BigInt::from(24i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(19i32), BigInt::from(31i32)).unwrap();

        assert_eq!(a * b, FieldElement::new(BigInt::from(22i32), BigInt::from(31i32)).unwrap());
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(BigInt::from(17i32), BigInt::from(31i32)).unwrap();
        assert_eq!(a.pow(&BigInt::from(3)), FieldElement::new(BigInt::from(15i32), BigInt::from(31i32)).unwrap());

        let a = FieldElement::new(BigInt::from(5i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(18i32), BigInt::from(31i32)).unwrap();
        assert_eq!(a.pow(&BigInt::from(5i32)) * b, FieldElement::new(BigInt::from(16i32), BigInt::from(31i32)).unwrap());
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(BigInt::from(3i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(24i32), BigInt::from(31i32)).unwrap();
        assert_eq!(a / b, FieldElement::new(BigInt::from(4i32), BigInt::from(31i32)).unwrap());

        let a = FieldElement::new(BigInt::from(17i32), BigInt::from(31i32)).unwrap();
        assert_eq!(a.pow(&BigInt::from(-3i32)), FieldElement::new(BigInt::from(29i32), BigInt::from(31i32)).unwrap());

        let b = FieldElement::new(BigInt::from(4i32), BigInt::from(31i32)).unwrap();
        let b = FieldElement::new(BigInt::from(11i32), BigInt::from(31i32)).unwrap();
        assert_eq!(a.pow(&BigInt::from(-4i32)) * b, FieldElement::new(BigInt::from(13i32), BigInt::from(31i32)).unwrap());
    }
}
