use rug::{rand::RandState, Integer};

use crate::elliptic_curve::point::Point;

use super::signature::Signature;

static N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

#[derive(PartialEq, Debug, Clone)]
pub struct PrivateKey {
    secret: Integer,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> PrivateKey {
        PrivateKey {
            secret: secret.clone(),
            point: secret * Point::g_point(),
        }
    }

    pub fn secret(self) -> Integer {
        self.secret
    }

    pub fn point(self) -> Point {
        self.point
    }

    pub fn sign(self, z: Integer) -> Signature {
        let n = Integer::from_str_radix(N, 16).unwrap();
        let mut rand = RandState::new();

        let k = n.clone().random_below(&mut rand);

        let r = (k.clone() * Point::g_point()).x().unwrap().num();
        let k_inv = k.pow_mod(&(n.clone() - 2i32), &n).unwrap();
        let mut s = (z + r.clone() * self.secret) * k_inv % &n;
        if s > n.clone() / 2i32 {
            s = n - s;
        }
        Signature::new(r, s)
    }
}

#[cfg(test)]
mod tests {
    use rug::ops::Pow;

    use super::*;

    #[test]
    fn test_sign() {
        let n = Integer::from_str_radix(N, 16).unwrap();
        let mut rand = RandState::new();

        let pk = PrivateKey::new(n.random_below(&mut rand));
        let z = (Integer::from(2i32).pow(256)).random_below(&mut RandState::new());
        let sig = pk.clone().sign(z.clone());
        assert!(pk.point().verify(z, sig));
    }
}
