use std::ops::Add;

use crate::finite_field::field_element::FieldElement;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: FieldElement,
    y: FieldElement,
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
            println!("y**2 = {}", y.pow(2));
            println!("lo otro = {}", x.pow(3) + a * x + b);
            return Err(PointError::PointNotInCurve(format!(
                "({}, {}) is not on the curve",
                x, y,
            )));
        }

        Ok(Point { a, b, x, y })
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "{}",
                format!("Points {:?}, {:?} are not on the same curve.", self, rhs)
            );
        }

        let x_sum;
        let y_sum;
        if self == rhs {
            let slope = (FieldElement::new(3, self.a.prime()).unwrap() * self.x.pow(2) + self.a)
                / (FieldElement::new(2, self.a.prime()).unwrap() * self.y);
            x_sum = slope.pow(2) - self.x - rhs.x;
            y_sum = slope * (self.x - x_sum) - self.y;
        } else {
            let slope = (rhs.y - self.y) / (rhs.x - self.x);
            x_sum = slope.pow(2) - self.x - rhs.x;
            y_sum = slope * (self.x - x_sum) - self.y;
        }

        Point {
            a: self.a,
            b: self.b,
            x: x_sum,
            y: y_sum,
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
