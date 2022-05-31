use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

#[derive(Debug)]
pub enum FieldElementError {
    FieldRangeError(String),
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<FieldElement, FieldElementError> {
        if num >= prime || num < 0 {
            return Err(FieldElementError::FieldRangeError(format!(
                "Num {} not in field range 0 to {}",
                num,
                prime - 1
            )));
        }

        Ok(FieldElement { num, prime })
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}
