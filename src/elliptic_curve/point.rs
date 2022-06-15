use std::{
    fmt,
    ops::{Add, Mul},
};

use rug::{ops::Pow, Integer};

use crate::{cryptography::signature::Signature, finite_field::field_element::FieldElement};

static N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

#[derive(Debug)]
pub enum PointError {
    PointNotInCurve(String),
}

impl Point {
    pub fn new(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Point, PointError> {
        if y.pow(&Integer::from(2i32))
            != x.pow(&Integer::from(3i32)) + a.clone() * x.clone() + b.clone()
        {
            return Err(PointError::PointNotInCurve(format!(
                "({}, {}) is not on the curve",
                x, y,
            )));
        }

        Ok(Point {
            a,
            b,
            x: Some(x),
            y: Some(y),
        })
    }

    pub fn infinity(a: FieldElement, b: FieldElement) -> Point {
        Point {
            x: None,
            y: None,
            a,
            b,
        }
    }

    pub fn g_point() -> Point {
        let gx = Integer::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        )
        .unwrap();
        let gy = Integer::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        )
        .unwrap();
        let p = Integer::from(2i32).pow(256) - Integer::from(2i32).pow(32) - Integer::from(977i32);

        let x = FieldElement::new(gx, p.clone()).unwrap();
        let y = FieldElement::new(gy, p.clone()).unwrap();
        let seven = FieldElement::new(Integer::from(7i32), p.clone()).unwrap();
        let zero = FieldElement::new(Integer::from(0i32), p).unwrap();

        Point {
            x: Some(x),
            y: Some(y),
            a: zero,
            b: seven,
        }
    }

    pub fn verify(self, z: Integer, sig: Signature) -> bool {
        let n = Integer::from_str_radix(N, 16).unwrap();
        let s_inv = sig
            .clone()
            .s()
            .pow_mod(&(n.clone() - Integer::from(2i32)), &n)
            .unwrap();
        let u = z * s_inv.clone() % n.clone();
        let v = sig.clone().r() * s_inv % n;
        let total = u * Point::g_point() + v * self;
        total.x.unwrap().num() == sig.r()
    }

    pub fn x(self) -> Option<FieldElement> {
        self.x
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point({}, {})_{}_{}",
            self.x.clone().unwrap(),
            self.y.clone().unwrap(),
            self.a,
            self.b
        )
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        if self.x.is_none() {
            return rhs;
        } else if rhs.x.is_none() {
            return self;
        }

        match (
            self.x.as_ref(),
            self.y.as_ref(),
            rhs.x.as_ref(),
            rhs.y.as_ref(),
        ) {
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                let x1 = x1.clone();
                let y1 = y1.clone();
                let x2 = x2.clone();
                let y2 = y2.clone();

                if self.a != rhs.a || self.b != rhs.b {
                    panic!(
                        "{}",
                        format!("Points {:?}, {:?} are not on the same curve.", self, rhs)
                    );
                }

                let x_sum;
                let y_sum;
                if self == rhs {
                    if y1 == FieldElement::new(Integer::from(0i32), y1.clone().prime()).unwrap() {
                        return Point {
                            x: None,
                            y: None,
                            a: self.a,
                            b: self.b,
                        };
                    }
                    let slope = (FieldElement::new(Integer::from(3i32), self.a.clone().prime())
                        .unwrap()
                        * x1.pow(&Integer::from(2i32))
                        + self.a.clone())
                        / (FieldElement::new(Integer::from(2i32), self.a.clone().prime()).unwrap()
                            * y1.clone());
                    x_sum = slope.pow(&Integer::from(2i32)) - x1.clone() - x2;
                    y_sum = slope * (x1 - x_sum.clone()) - y1;
                } else {
                    let slope = (y2 - y1.clone()) / (x2.clone() - x1.clone());
                    x_sum = slope.pow(&Integer::from(2i32)) - x1.clone() - x2;
                    y_sum = slope * (x1 - x_sum.clone()) - y1;
                }

                Point {
                    a: self.a,
                    b: self.b,
                    x: Some(x_sum),
                    y: Some(y_sum),
                }
            }

            _ => Point::infinity(self.a, self.b),
        }
    }
}

impl Mul<Point> for Integer {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        let mut coef = self;
        let mut current = point.clone();
        let mut result = Point::infinity(point.a, point.b);

        while coef > 0 {
            if coef.is_odd() {
                result = result + current.clone(); // TODO: fix when MulAssign is implemented
            }
            current = current.clone() + current; // TODO: fix when AddAssign is implemented
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = 0;
        let b = 7;

        assert!(point(192, 105, a, b, prime).is_ok());
        assert!(point(17, 56, a, b, prime).is_ok());
        assert!(point(200, 119, a, b, prime).is_err());
        assert!(point(1, 193, a, b, prime).is_ok());
        assert!(point(42, 99, a, b, prime).is_err());
    }

    #[test]
    fn test_add() {
        let prime = 223;
        let a = 0;
        let b = 7;

        let p1 = point(192, 105, a, b, prime).unwrap();
        let p2 = point(17, 56, a, b, prime).unwrap();
        let p3 = point(170, 142, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);

        let p1 = point(47, 71, a, b, prime).unwrap();
        let p2 = point(117, 141, a, b, prime).unwrap();
        let p3 = point(60, 139, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);

        let p1 = point(143, 98, a, b, prime).unwrap();
        let p2 = point(76, 66, a, b, prime).unwrap();
        let p3 = point(47, 71, a, b, prime).unwrap();

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_mul() {
        let prime = 223;
        let a = 0;
        let b = 7;

        let p1 = point(170, 142, a, b, prime).unwrap();
        assert_eq!(p1.clone() + p1.clone(), Integer::from(2) * p1.clone());
        assert_eq!(
            Integer::from(2) * p1.clone() + p1.clone(),
            Integer::from(3) * p1
        );
    }

    fn point(x: i128, y: i128, a: i128, b: i128, prime: i128) -> Result<Point, PointError> {
        Point::new(
            FieldElement::new(Integer::from(x), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(y), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(a), Integer::from(prime)).unwrap(),
            FieldElement::new(Integer::from(b), Integer::from(prime)).unwrap(),
        )
    }
}
