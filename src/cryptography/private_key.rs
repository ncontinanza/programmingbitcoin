use std::fmt;

use rand::distributions::{ Distribution, Uniform };
use rug::Integer;

use crate::secp256k1::s_256_point::S256Point;

use super::signature::Signature;

static N: &str = "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";


#[derive(PartialEq, Debug, Clone)]
pub struct PrivateKey {
    secret: Integer,
    point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> PrivateKey {
        PrivateKey { secret: secret.clone(), point: secret * S256Point::g_point() }
    }

    pub fn secret(self) -> Integer {
        self.secret
    }

    pub fn point(self) -> S256Point {
        self.point
    }

    pub fn sign(self, z: Integer) -> Signature {
        let n = Integer::from_str_radix(N, 16).unwrap();
        let k = Integer::from(Uniform::from(0..=n.to_i32().unwrap()).sample(&mut rand::thread_rng()));
        let r = (k.clone() * S256Point::g_point()).x().unwrap().num();
        let k_inv = k.pow_mod(&(n.clone() - 2i32), &n).unwrap();
        let mut s = (z + r.clone() * self.secret) * k_inv % &n;
        if &s > &(n.clone() / 2i32) {
            s = n - s;
        }
        Signature::new(r, s)

        
    }
}

