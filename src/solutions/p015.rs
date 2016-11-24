use math::numeric;

use solutions::Solution;

pub fn solve() -> Solution {
    // LOL
    Solution::new(&format!("{}", numeric::binomial(&40u64, &20).unwrap()))
}
