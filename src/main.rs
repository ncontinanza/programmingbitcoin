use programmingbitcoin::finite_field::field_element::{FieldElement, FieldElementError};

fn main() -> Result<(), FieldElementError> {
    // Exercise 2:
    let a = FieldElement::new(44, 57)?;
    let b = FieldElement::new(33, 57)?;

    assert_eq!(a + b, FieldElement::new(20, 57)?);

    let a = FieldElement::new(9, 57)?;
    let b = FieldElement::new(29, 57)?;

    assert_eq!(a - b, FieldElement::new(37, 57)?);

    let a = FieldElement::new(17, 57)?;
    let b = FieldElement::new(42, 57)?;
    let c = FieldElement::new(49, 57)?;

    assert_eq!(a + b + c, FieldElement::new(51, 57)?);

    let a = FieldElement::new(52, 57)?;
    let b = FieldElement::new(30, 57)?;
    let c = FieldElement::new(38, 57)?;

    assert_eq!(a - b - c, FieldElement::new(41, 57)?);

    // Exercise 4:
    let a = FieldElement::new(95, 97)?;
    let b = FieldElement::new(45, 97)?;
    let c = FieldElement::new(31, 97)?;

    assert_eq!(a * b * c, FieldElement::new(23, 97)?);

    let a = FieldElement::new(17, 97)?;
    let b = FieldElement::new(13, 97)?;
    let c = FieldElement::new(19, 97)?;
    let d = FieldElement::new(44, 97)?;

    assert_eq!(a * b * c * d, FieldElement::new(68, 97)?);

    let a = FieldElement::new(12, 97)?;
    let b = FieldElement::new(77, 97)?;

    assert_eq!(a.pow(7) * b.pow(49), FieldElement::new(63, 97)?);

    // Exercise 8
    let a = FieldElement::new(3, 31)?;
    let b = FieldElement::new(24, 31)?;

    assert_eq!(a / b, FieldElement::new(4, 31)?);

    let a = FieldElement::new(17, 31)?;
    assert_eq!(a.pow(-3), FieldElement::new(29, 31)?);

    let a = FieldElement::new(4, 31)?;
    let b = FieldElement::new(11, 31)?;

    assert_eq!(a.pow(-4) * b, FieldElement::new(13, 31)?);

    Ok(())
}
