use std::ops::Add;

use crate::finite_field::field_element::FieldElement;

#[derive(PartialEq, Debug, Clone, Copy)]
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
        if y.pow(2) != x.pow(3) + a * x + b {
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

    pub fn rmul(self, coefficient: i32) -> Self {
        self
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
                    if *y1 == FieldElement::new(0, y1.prime()).unwrap() {
                        return Point {
                            x: None,
                            y: None,
                            a: self.a,
                            b: self.b,
                        };
                    }
                    let slope = (FieldElement::new(3, self.a.prime()).unwrap() * x1.pow(2)
                        + self.a)
                        / (FieldElement::new(2, self.a.prime()).unwrap() * *y1);
                    x_sum = slope.pow(2) - *x1 - *x2;
                    y_sum = slope * (*x1 - x_sum) - *y1;
                } else {
                    let slope = (*y2 - *y1) / (*x2 - *x1);
                    x_sum = slope.pow(2) - *x1 - *x2;
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

    fn point(x: i128, y: i128, a: i128, b: i128, prime: i128) -> Result<Point, PointError> {
        Point::new(
            FieldElement::new(x, prime).unwrap(),
            FieldElement::new(y, prime).unwrap(),
            FieldElement::new(a, prime).unwrap(),
            FieldElement::new(b, prime).unwrap(),
        )
    }
}