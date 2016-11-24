use solutions::Solution;

pub fn solve() -> Solution {
    let mut sum = 0u64;
    let mut sum_sq = 0u64;

    for n in 1..101 {
        sum += n;
        sum_sq += n * n;
    }

    Solution::new(&format!("{}", sum * sum - sum_sq))
}
