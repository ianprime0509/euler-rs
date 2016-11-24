use num::Integer;

use solutions::Solution;

pub fn solve() -> Solution {
    let mut lcm: u64 = 2;
    for n in 3..21 {
        lcm = lcm.lcm(&n);
    }

    Solution::new(&format!("{}", lcm))
}
