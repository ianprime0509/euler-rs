use std::ops::Mul;
use num::{BigInt, Integer, FromPrimitive};

use math::numeric;

pub fn solve() {
    let n: BigInt = FromPrimitive::from_u32(100).unwrap();
    println!("{}", numeric::sum_digits(&factorial(&n)));
}

fn factorial<T>(n: &T) -> T
    where T: Integer + Clone,
          for<'a> &'a T: Mul<Output = T>
{
    let mut fact = T::one();
    let mut m = T::one();
    while &m <= n {
        fact = &fact * &m;
        m = m + T::one();
    }

    fact
}
