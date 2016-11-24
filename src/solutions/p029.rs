use std::collections::HashSet;

use num::{BigUint, FromPrimitive};

use math::numeric;
use solutions::Solution;

pub fn solve() -> Solution {
    // This is a very brute force way of doing things,
    // but whatever, it runs quickly enough :^)
    let mut powers = HashSet::new();
    for a in 2..101 {
        for b in 2..101u32 {
            let a: BigUint = FromPrimitive::from_u32(a).unwrap();
            powers.insert(numeric::pow_primint(&a, b));
        }
    }

    Solution::new(&format!("{}", powers.len()))
}
