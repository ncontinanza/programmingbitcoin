#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Secp256k1 {
    a: FieldElement,
    b: FieldElement,
    prime: i128,
    g: Point,
    n: i128,
}

impl Secp256k1 {
    pub fn new(a: FieldElement, b: FieldElement, prime: i128, g: Point, n: i128) {
        Secp256k1 {
            a,
            b,
            prime,
            g,
            n,
        }

    }
}