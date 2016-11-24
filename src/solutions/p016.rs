use num::{BigInt, One};

use math::numeric;
use solutions::Solution;

pub fn solve() -> Solution {
    let two = BigInt::one() + BigInt::one();
    let n = numeric::pow_primint(&two, 1000u32);

    Solution::new(&format!("{}", numeric::sum_digits(&n)))
}
