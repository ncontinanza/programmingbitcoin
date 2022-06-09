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

    // CHAPTER 3: Elliptic Curve Cryptography
    let x1 = FieldElement::new(170, 223).unwrap();
    let y1 = FieldElement::new(142, 223).unwrap();

    let x2 = FieldElement::new(60, 223).unwrap();
    let y2 = FieldElement::new(139, 223).unwrap();

    let p1 = Point::new(
        x1,
        y1,
        FieldElement::new(0, 223).unwrap(),
        FieldElement::new(7, 223).unwrap(),
    )
    .unwrap();
    let p2 = Point::new(
        x2,
        y2,
        FieldElement::new(0, 223).unwrap(),
        FieldElement::new(7, 223).unwrap(),
    )
    .unwrap();

    assert_eq!(
        p1 + p2,
        Point::new(
            FieldElement::new(220, 223).unwrap(),
            FieldElement::new(181, 223).unwrap(),
            FieldElement::new(0, 223).unwrap(),
            FieldElement::new(7, 223).unwrap()
        )
        .unwrap()
    );

    // Working with secp256k1:

    let n = i128::from_str_radix(
        "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    let g = Point::new(
        FieldElement::new(
            i128::from_str_radix(
                "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .unwrap(),
            n,
        )
        .unwrap(),
        FieldElement::new(
            i128::from_str_radix(
                "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .unwrap(),
            n,
        )
        .unwrap(),
        FieldElement::new(0, 223).unwrap(),
        FieldElement::new(7, 223).unwrap(),
    )
    .unwrap();
    println!("N * G = {}", n * g);
}
