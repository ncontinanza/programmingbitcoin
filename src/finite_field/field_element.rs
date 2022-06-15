use rug::Integer;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Clone)]
pub struct FieldElement {
    num: Integer,
    prime: Integer,
}

#[derive(PartialEq, Debug)]
pub enum FieldElementError {
    FieldRangeError(String),
    FieldOrderError(String),
}

impl FieldElement {
    pub fn new(num: Integer, prime: Integer) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < 0 {
            return Err(FieldElementError::FieldRangeError(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn pow(&self, exp: &Integer) -> FieldElement {
        if self.prime == 1i32 {
            FieldElement {
                num: Integer::from(0i32),
                prime: self.prime.clone(),
            }
        } else if let Some(result) = self.num.pow_mod_ref(exp, &self.prime) {
            FieldElement {
                num: Integer::from(result),
                prime: self.prime.clone(),
            }
        } else {
            unreachable!()
        }
    }

    pub fn prime(self) -> Integer {
        self.prime
    }

    pub fn num(self) -> Integer {
        self.num
    }

    fn value_of(&self, value: Integer) -> Integer {
        if value < 0 || value >= self.prime {
            let result = value.div_rem_euc_ref(&self.prime);
            let (_, normalized_value) = <(Integer, Integer)>::from(result);
            normalized_value
        } else {
            value
        }
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
        let sum = &self.num + rhs.num;
        if sum < 0 || sum >= self.prime {
            FieldElement {
                num: self.value_of(sum),
                prime: self.prime,
            }
        } else {
            FieldElement {
                num: sum,
                prime: self.prime,
            }
        }
    }
    // TODO: implement add() with references, to remove clones.
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot substract two numbers in different fields"
        );

        FieldElement {
            num: self.value_of(&self.num - rhs.num),
            prime: self.prime,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        let zero = FieldElement {
            num: Integer::from(0i32),
            prime: self.prime.clone(),
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
            num: self.value_of(self.num.clone() * rhs.num),
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

        assert_ne!(Integer::from(0i32), rhs.num, "Zero is not a valid divisor!");
        self.clone() * rhs.pow(&(self.prime - Integer::from(2i32)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let c = FieldElement::new(Integer::from(15i32), Integer::from(31i32)).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(15i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a + b,
            FieldElement::new(Integer::from(17i32), Integer::from(31i32)).unwrap()
        );

        let a = FieldElement::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(21i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a + b,
            FieldElement::new(Integer::from(7i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::new(Integer::from(29i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(4i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a - b,
            FieldElement::new(Integer::from(25i32), Integer::from(31i32)).unwrap()
        );

        let a = FieldElement::new(Integer::from(15i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(30i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a - b,
            FieldElement::new(Integer::from(16i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_neg() {
        let a = FieldElement::new(Integer::from(9i32), Integer::from(19i32)).unwrap();

        assert_eq!(
            &(-a.clone()),
            &FieldElement::new(Integer::from(10i32), Integer::from(19i32)).unwrap()
        );
        assert_eq!(
            -a.clone() + a,
            FieldElement::new(Integer::from(0i32), Integer::from(19i32)).unwrap()
        );
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(Integer::from(24i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(19i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a * b,
            FieldElement::new(Integer::from(22i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(3i32)),
            FieldElement::new(Integer::from(15i32), Integer::from(31i32)).unwrap()
        );

        let a = FieldElement::new(Integer::from(5i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(18i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(5i32)) * b,
            FieldElement::new(Integer::from(16i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_div() {
        let a = FieldElement::new(Integer::from(3i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(24i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a / b,
            FieldElement::new(Integer::from(4i32), Integer::from(31i32)).unwrap()
        );

        let a = FieldElement::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(-3i32)),
            FieldElement::new(Integer::from(29i32), Integer::from(31i32)).unwrap()
        );

        let a = FieldElement::new(Integer::from(4i32), Integer::from(31i32)).unwrap();
        let b = FieldElement::new(Integer::from(11i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(-4i32)) * b,
            FieldElement::new(Integer::from(13i32), Integer::from(31i32)).unwrap()
        );
    }
}
