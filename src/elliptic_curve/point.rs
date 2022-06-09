use std::{
    fmt,
    ops::{Add, Mul},
};

use num_bigint::BigInt;

use crate::finite_field::field_element::FieldElement;

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
        if y.pow(&BigInt::from(2i32)) != x.pow(&BigInt::from(3i32)) + a.clone() * x.clone() + b.clone() {
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
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Point({},{})_{}_{}",
            self.x.as_ref().unwrap(),
            self.y.as_ref().unwrap(),
            self.a,
            self.b
        )
    }
}

// TODO: Implement AddAssign, SubAssign, MulAssign and DivAssign
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
                if self.a != rhs.a || self.b != rhs.b {
                    panic!(
                        "{}",
                        format!("Points {:?}, {:?} are not on the same curve.", self, rhs)
                    );
                }

                let x_sum;
                let y_sum;
                if self == rhs {
                    if *y1 == FieldElement::new(BigInt::from(0i32), y1.prime()).unwrap() {
                        return Point {
                            x: None,
                            y: None,
                            a: self.a,
                            b: self.b,
                        };
                    }
                    let slope = (FieldElement::new(BigInt::from(3i32), self.a.prime()).unwrap() * x1.pow(&BigInt::from(2i32))
                        + self.a)
                        / (FieldElement::new(BigInt::from(2i32), self.a.prime()).unwrap() * *y1);
                    x_sum = slope.pow(&BigInt::from(2i32)) - *x1 - *x2;
                    y_sum = slope * (*x1 - x_sum) - *y1;
                } else {
                    let slope = (*y2 - *y1) / (*x2 - *x1);
                    x_sum = slope.pow(&BigInt::from(2i32)) - *x1 - *x2;
                    y_sum = slope * (*x1 - x_sum) - *y1;
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

impl Mul<Point> for i128 {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        let mut coef = self;
        let mut current = point;
        let mut result = Point::infinity(point.a, point.b);

        while coef > 0 {
            if coef % 2 != 0 {
                result = result + current; // TODO: fix when MulAssign is implemented
            }
            current = current + current; // TODO: fix when AddAssign is implemented
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: make these tests really unit tests
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
        assert_eq!(p1 + p1, 2 * p1);
        assert_eq!(2 * p1 + p1, 3 * p1);
    }

    fn point(x: i32, y: i32, a: i32, b: i32, prime: i32) -> Result<Point, PointError> {
        Point::new(
            FieldElement::new(BigInt::from(x), BigInt::from(prime)).unwrap(),
            FieldElement::new(BigInt::from(x), BigInt::from(prime)).unwrap(),
            FieldElement::new(BigInt::from(x), BigInt::from(prime)).unwrap(),
            FieldElement::new(BigInt::from(x), BigInt::from(prime)).unwrap(),
        )
    }
}
