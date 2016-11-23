//! Modular arithmetic
//! The `Mod` type is supposed to work as it does in PARI/GP
use std::ops::{Add, Sub, Mul, Neg};
use num::{PrimInt, Integer, Signed};

#[derive(Clone, Copy)]
pub struct Mod<T: PrimInt + Integer>(T, T);

impl<T> Mod<T> where T: PrimInt + Integer {}

impl<T> Add for Mod<T>
    where T: PrimInt + Integer
{
    type Output = Mod<T>;

    fn add(self, other: Mod<T>) -> Mod<T> {
        let gcd = if self.1 == other.1 {
            self.1
        } else {
            self.1.gcd(&other.1)
        };

        Mod((self.0 + other.0) % gcd, gcd)
    }
}

impl<T> Neg for Mod<T>
    where T: PrimInt + Integer + Signed
{
    type Output = Mod<T>;

    fn neg(self) -> Mod<T> {
        Mod(-self.0 + self.1, self.1)
    }
}

impl<T> Sub for Mod<T>
    where T: PrimInt + Integer
{
    type Output = Mod<T>;

    fn sub(self, other: Mod<T>) -> Mod<T> {
        let gcd = if self.1 == other.1 {
            self.1
        } else {
            self.1.gcd(&other.1)
        };

        Mod((self.0 + gcd - other.0) % gcd, gcd)
    }
}

impl<T> Mul for Mod<T>
    where T: PrimInt + Integer
{
    type Output = Mod<T>;

    fn mul(self, other: Mod<T>) -> Mod<T> {
        let gcd = if self.1 == other.1 {
            self.1
        } else {
            self.1.gcd(&other.1)
        };

        Mod((self.0 * other.0) % gcd, gcd)
    }
}
