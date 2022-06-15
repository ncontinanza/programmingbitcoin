use rug::Integer;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Clone)]
pub struct S256Field {
    num: Integer,
    prime: Integer,
}

#[derive(PartialEq, Debug)]
pub enum S256FieldError {
    FieldRangeError(String),
    FieldOrderError(String),
}

impl S256Field {
    pub fn new(num: Integer, prime: Integer) -> Result<S256Field, S256FieldError> {
        if num >= prime || num < 0 {
            return Err(S256FieldError::FieldRangeError(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }

        Ok(S256Field { num, prime })
    }

    pub fn pow(&self, exp: &Integer) -> S256Field {
        if self.prime == 1i32 {
            S256Field {
                num: Integer::from(0i32),
                prime: self.prime.clone(),
            }
        } else if let Some(result) = self.num.pow_mod_ref(exp, &self.prime) {
            S256Field {
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

impl fmt::Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl Add for S256Field {
    type Output = S256Field;

    fn add(self, rhs: S256Field) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot add two numbers in different fields"
        );
        let sum = &self.num + rhs.num;
        if sum < 0 || sum >= self.prime {
            S256Field {
                num: self.value_of(sum),
                prime: self.prime,
            }
        } else {
            S256Field {
                num: sum,
                prime: self.prime,
            }
        }
    }
    // TODO: implement add() with references, to remove clones.
}

impl Sub for S256Field {
    type Output = S256Field;

    fn sub(self, rhs: S256Field) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot substract two numbers in different fields"
        );

        S256Field {
            num: self.value_of(&self.num - rhs.num),
            prime: self.prime,
        }
    }
}

impl Neg for S256Field {
    type Output = S256Field;

    fn neg(self) -> Self::Output {
        let zero = S256Field {
            num: Integer::from(0i32),
            prime: self.prime.clone(),
        };
        zero - self
    }
}

impl Mul for S256Field {
    type Output = S256Field;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.prime, rhs.prime,
            "Cannot multiply two numbers in different fields"
        );

        S256Field {
            num: self.value_of(self.num.clone() * rhs.num),
            prime: self.prime,
        }
    }
}

impl Div for S256Field {
    type Output = S256Field;

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
        let a = S256Field::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let c = S256Field::new(Integer::from(15i32), Integer::from(31i32)).unwrap();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_add() {
        let a = S256Field::new(Integer::from(2i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(15i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a + b,
            S256Field::new(Integer::from(17i32), Integer::from(31i32)).unwrap()
        );

        let a = S256Field::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(21i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a + b,
            S256Field::new(Integer::from(7i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_sub() {
        let a = S256Field::new(Integer::from(29i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(4i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a - b,
            S256Field::new(Integer::from(25i32), Integer::from(31i32)).unwrap()
        );

        let a = S256Field::new(Integer::from(15i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(30i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a - b,
            S256Field::new(Integer::from(16i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_neg() {
        let a = S256Field::new(Integer::from(9i32), Integer::from(19i32)).unwrap();

        assert_eq!(
            &(-a.clone()),
            &S256Field::new(Integer::from(10i32), Integer::from(19i32)).unwrap()
        );
        assert_eq!(
            -a.clone() + a,
            S256Field::new(Integer::from(0i32), Integer::from(19i32)).unwrap()
        );
    }

    #[test]
    fn test_mul() {
        let a = S256Field::new(Integer::from(24i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(19i32), Integer::from(31i32)).unwrap();

        assert_eq!(
            a * b,
            S256Field::new(Integer::from(22i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_pow() {
        let a = S256Field::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(3i32)),
            S256Field::new(Integer::from(15i32), Integer::from(31i32)).unwrap()
        );

        let a = S256Field::new(Integer::from(5i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(18i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(5i32)) * b,
            S256Field::new(Integer::from(16i32), Integer::from(31i32)).unwrap()
        );
    }

    #[test]
    fn test_div() {
        let a = S256Field::new(Integer::from(3i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(24i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a / b,
            S256Field::new(Integer::from(4i32), Integer::from(31i32)).unwrap()
        );

        let a = S256Field::new(Integer::from(17i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(-3i32)),
            S256Field::new(Integer::from(29i32), Integer::from(31i32)).unwrap()
        );

        let a = S256Field::new(Integer::from(4i32), Integer::from(31i32)).unwrap();
        let b = S256Field::new(Integer::from(11i32), Integer::from(31i32)).unwrap();
        assert_eq!(
            a.pow(&Integer::from(-4i32)) * b,
            S256Field::new(Integer::from(13i32), Integer::from(31i32)).unwrap()
        );
    }
}
