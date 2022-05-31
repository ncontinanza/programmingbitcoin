use programmingbitcoin::FieldElement;

fn main() {
    let a = FieldElement::new(7, 13).unwrap();
    let b = FieldElement::new(6, 13).unwrap();

    assert_ne!(a, b);
}
