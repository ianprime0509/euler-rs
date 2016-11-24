use num::{BigInt, One};
use solutions::Solution;

pub fn solve() -> Solution {
    let mut n = BigInt::one();
    let mut m = BigInt::one();

    // The variable i keeps track of the index of m in the series
    for i in 2.. {
        // When we find a term with 1000 digits, print out the index
        // and stop
        if m.to_string().len() == 1000 {
            return Solution::new(&format!("{}", i));
        }

        // Get next pair of sequence elements and continue
        let tmp = n.clone();
        n = m;
        m = &n + &tmp;
    }

    Solution::new("Something is wrong")
}
