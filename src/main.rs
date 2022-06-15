use programmingbitcoin::secp256k1::s_256_field::S256Field;
use programmingbitcoin::secp256k1::s_256_point::S256Point;
use programmingbitcoin::{elliptic_curve::point::Point, finite_field::field_element::FieldElement};
use rug::ops::*;
use rug::Integer;

fn main() {
    // CHAPTER 1: Finite Fields
    // Exercise 2:
    let a = FieldElement::new(Integer::from(44i32), Integer::from(57i32)).unwrap();
    let b = FieldElement::new(Integer::from(33i32), Integer::from(57i32)).unwrap();

    assert_eq!(
        a + b,
        FieldElement::new(Integer::from(20i32), Integer::from(57i32)).unwrap()
    );

    let a = FieldElement::new(Integer::from(9i32), Integer::from(57i32)).unwrap();
    let b = FieldElement::new(Integer::from(29i32), Integer::from(57i32)).unwrap();

    assert_eq!(
        a - b,
        FieldElement::new(Integer::from(37i32), Integer::from(57i32)).unwrap()
    );

    let a = FieldElement::new(Integer::from(17i32), Integer::from(57i32)).unwrap();
    let b = FieldElement::new(Integer::from(42i32), Integer::from(57i32)).unwrap();
    let c = FieldElement::new(Integer::from(49i32), Integer::from(57i32)).unwrap();

    assert_eq!(
        a + b + c,
        FieldElement::new(Integer::from(51i32), Integer::from(57i32)).unwrap()
    );

    let a = FieldElement::new(Integer::from(52i32), Integer::from(57i32)).unwrap();
    let b = FieldElement::new(Integer::from(30i32), Integer::from(57i32)).unwrap();
    let c = FieldElement::new(Integer::from(38i32), Integer::from(57i32)).unwrap();

    assert_eq!(
        a - b - c,
        FieldElement::new(Integer::from(41i32), Integer::from(57i32)).unwrap()
    );

    // Exercise 4:
    let a = FieldElement::new(Integer::from(95i32), Integer::from(97i32)).unwrap();
    let b = FieldElement::new(Integer::from(45i32), Integer::from(97i32)).unwrap();
    let c = FieldElement::new(Integer::from(31i32), Integer::from(97i32)).unwrap();

    assert_eq!(
        a * b * c,
        FieldElement::new(Integer::from(23i32), Integer::from(97i32)).unwrap()
    );

    let a = FieldElement::new(Integer::from(17i32), Integer::from(97i32)).unwrap();
    let b = FieldElement::new(Integer::from(13i32), Integer::from(97i32)).unwrap();
    let c = FieldElement::new(Integer::from(19i32), Integer::from(97i32)).unwrap();
    let d = FieldElement::new(Integer::from(44i32), Integer::from(97i32)).unwrap();

    assert_eq!(
        a * b * c * d,
        FieldElement::new(Integer::from(68i32), Integer::from(97i32)).unwrap()
    );

    let a = FieldElement::new(Integer::from(12i32), Integer::from(97i32)).unwrap();
    let b = FieldElement::new(Integer::from(77i32), Integer::from(97i32)).unwrap();

    assert_eq!(
        a.pow(&Integer::from(7i32)) * b.pow(&Integer::from(49i32)),
        FieldElement::new(Integer::from(63i32), Integer::from(97i32)).unwrap()
    );

    // Exercise 8
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

    // CHAPTER 3: Elliptic Curve Cryptography
    let x1 = FieldElement::new(Integer::from(170i32), Integer::from(223i32)).unwrap();
    let y1 = FieldElement::new(Integer::from(142i32), Integer::from(223i32)).unwrap();

    let x2 = FieldElement::new(Integer::from(60i32), Integer::from(223i32)).unwrap();
    let y2 = FieldElement::new(Integer::from(139i32), Integer::from(223i32)).unwrap();

    let p1 = Point::new(
        x1,
        y1,
        FieldElement::new(Integer::from(0i32), Integer::from(223i32)).unwrap(),
        FieldElement::new(Integer::from(7i32), Integer::from(223i32)).unwrap(),
    )
    .unwrap();
    let p2 = Point::new(
        x2,
        y2,
        FieldElement::new(Integer::from(0i32), Integer::from(223i32)).unwrap(),
        FieldElement::new(Integer::from(7i32), Integer::from(223i32)).unwrap(),
    )
    .unwrap();

    assert_eq!(
        p1 + p2,
        Point::new(
            FieldElement::new(Integer::from(220i32), Integer::from(223i32)).unwrap(),
            FieldElement::new(Integer::from(181i32), Integer::from(223i32)).unwrap(),
            FieldElement::new(Integer::from(0i32), Integer::from(223i32)).unwrap(),
            FieldElement::new(Integer::from(7i32), Integer::from(223i32)).unwrap()
        )
        .unwrap()
    );

    // Working with secp256k1:
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

    assert_eq!(gy.clone().pow(2) % &p, (gx.clone().pow(3) + 7i32) % &p);

    let n = Integer::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();

    println!("{}", n);

    let x = FieldElement::new(gx, p.clone()).unwrap();
    let y = FieldElement::new(gy, p.clone()).unwrap();
    let seven = FieldElement::new(Integer::from(7i32), p.clone()).unwrap();
    let zero = FieldElement::new(Integer::from(0i32), p).unwrap();

    let _g = Point::new(x, y, zero, seven).unwrap();

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
    let a = S256Field::new(Integer::from(0i32), p.clone()).unwrap();
    let b = S256Field::new(Integer::from(7i32), p.clone()).unwrap();
    let x = S256Field::new(gx, p.clone()).unwrap();
    let y = S256Field::new(gy, p).unwrap();

    let _s_256_point = S256Point::new(x, y, a, b);

    


}
