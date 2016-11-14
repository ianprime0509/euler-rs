use num::{BigInt, One};
use math::numeric;

pub fn solve() {
    let two = BigInt::one() + BigInt::one();
    let n = numeric::pow_primint(&two, 1000);

    println!("{}", numeric::sum_digits(&n));
}
