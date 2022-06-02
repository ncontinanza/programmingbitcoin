use programmingbitcoin::{elliptic_curve::point::Point, finite_field::field_element::FieldElement};

fn main() {
    // CHAPTER 1: Finite Fields
    // Exercise 2:
    let a = FieldElement::new(44, 57).unwrap();
    let b = FieldElement::new(33, 57).unwrap();

    assert_eq!(a + b, FieldElement::new(20, 57).unwrap());

    let a = FieldElement::new(9, 57).unwrap();
    let b = FieldElement::new(29, 57).unwrap();

    assert_eq!(a - b, FieldElement::new(37, 57).unwrap());

    let a = FieldElement::new(17, 57).unwrap();
    let b = FieldElement::new(42, 57).unwrap();
    let c = FieldElement::new(49, 57).unwrap();

    assert_eq!(a + b + c, FieldElement::new(51, 57).unwrap());

    let a = FieldElement::new(52, 57).unwrap();
    let b = FieldElement::new(30, 57).unwrap();
    let c = FieldElement::new(38, 57).unwrap();

    assert_eq!(a - b - c, FieldElement::new(41, 57).unwrap());

    // Exercise 4:
    let a = FieldElement::new(95, 97).unwrap();
    let b = FieldElement::new(45, 97).unwrap();
    let c = FieldElement::new(31, 97).unwrap();

    assert_eq!(a * b * c, FieldElement::new(23, 97).unwrap());

    let a = FieldElement::new(17, 97).unwrap();
    let b = FieldElement::new(13, 97).unwrap();
    let c = FieldElement::new(19, 97).unwrap();
    let d = FieldElement::new(44, 97).unwrap();

    assert_eq!(a * b * c * d, FieldElement::new(68, 97).unwrap());

    let a = FieldElement::new(12, 97).unwrap();
    let b = FieldElement::new(77, 97).unwrap();

    assert_eq!(a.pow(7) * b.pow(49), FieldElement::new(63, 97).unwrap());

    // Exercise 8
    let a = FieldElement::new(3, 31).unwrap();
    let b = FieldElement::new(24, 31).unwrap();

    assert_eq!(a / b, FieldElement::new(4, 31).unwrap());

    let a = FieldElement::new(17, 31).unwrap();
    assert_eq!(a.pow(-3), FieldElement::new(29, 31).unwrap());

    let a = FieldElement::new(4, 31).unwrap();
    let b = FieldElement::new(11, 31).unwrap();

    assert_eq!(a.pow(-4) * b, FieldElement::new(13, 31).unwrap());

    // CHAPTER 2: Elliptic Curves

    let _p1 = Point::new(-1.0, -1.0, 5.0, 7.0);
    let _p2 = Point::new(-1.0, -2.0, 5.0, 7.0);

    // Exercise 1
    let p1 = Point::new(2.0, 4.0, 5.0, 7.0);
    let p2 = Point::new(-1.0, -1.0, 5.0, 7.0);
    let p3 = Point::new(18.0, 77.0, 5.0, 7.0);
    let p4 = Point::new(5.0, 7.0, 5.0, 7.0);

    println!("p1 is on the curve? {}", p1.is_ok());
    println!("p2 is on the curve? {}", p2.is_ok());
    println!("p3 is on the curve? {}", p3.is_ok());
    println!("p4 is on the curve? {}", p4.is_ok());

    // Exercise 4
    let p1 = Point::new(2.0, 5.0, 5.0, 7.0).unwrap();
    let p2 = Point::new(-1.0, -1.0, 5.0, 7.0).unwrap();

    assert_eq!(p1 + p2, Point::new(3.0, -7.0, 5.0, 7.0).unwrap());

    // Exercise 6
    let p1 = Point::new(-1.0, -1.0, 5.0, 7.0).unwrap();

    assert_eq!(p1 + p1, Point::new(18.0, 77.0, 5.0, 7.0).unwrap());

    // CHAPTER 3: Elliptic Curve Cryptography
    
}
