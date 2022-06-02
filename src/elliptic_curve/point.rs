use std::{f32::INFINITY, ops::Add};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    a: f32,
    b: f32,
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub enum PointError {
    PointNotInCurve(String),
}

impl Point {
    pub fn new(x: f32, y: f32, a: f32, b: f32) -> Result<Point, PointError> {
        if x == INFINITY && y == INFINITY {
            return Ok(Point { a, b, x, y });
        }

        if y.powf(2.0) != x.powf(3.0) + a * x + b {
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
        let infinity = Point {
            a: self.a,
            b: self.b,
            x: INFINITY,
            y: INFINITY,
        };
        if self.x == INFINITY {
            return rhs;
        }
        if rhs.x == INFINITY {
            return self;
        }
        if self.x == rhs.x && self.y != rhs.y {
            return infinity;
        }

        let x_sum;
        let y_sum;
        if self == rhs {
            if self.y == 0.0 {
                return infinity;
            }
            let slope = (3.0 * self.x.powf(2.0) + self.a) / (2.0 * self.y);
            x_sum = slope.powf(2.0) - self.x - rhs.x;
            y_sum = slope * (self.x - x_sum) - self.y;
        } else {
            let slope = (rhs.y - self.y) / (rhs.x - self.x);
            x_sum = slope.powf(2.0) - self.x - rhs.x;
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
    fn test_add0() {
        let a = Point::new(INFINITY, INFINITY, 5.0, 7.0).unwrap();
        let b = Point::new(2.0, 5.0, 5.0, 7.0).unwrap();
        let c = Point::new(2.0, -5.0, 5.0, 7.0).unwrap();

        assert_eq!(a + b, b);
        assert_eq!(b + a, b);
        assert_eq!(b + c, a);
    }

    #[test]
    fn test_add1() {
        let a = Point::new(3.0, 7.0, 5.0, 7.0).unwrap();
        let b = Point::new(-1.0, -1.0, 5.0, 7.0).unwrap();

        assert_eq!(a + b, Point::new(2.0, -5.0, 5.0, 7.0).unwrap());
    }

    #[test]
    fn test_add2() {
        let a = Point::new(-1.0, -1.0, 5.0, 7.0).unwrap();

        assert_eq!(a + a, Point::new(18.0, 77.0, 5.0, 7.0).unwrap());
    }
}
