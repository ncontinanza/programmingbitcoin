use std::fmt;

use rug::Integer;

#[derive(PartialEq, Debug, Clone)]
pub struct Signature {
    r: Integer,
    s: Integer,
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Signature {
        Signature { r, s }
    }

    pub fn s(self) -> Integer {
        self.s
    }

    pub fn r(self) -> Integer {
        self.r
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature({},{})", self.r, self.s)
    }
}
