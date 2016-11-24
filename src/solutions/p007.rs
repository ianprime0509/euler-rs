use math::primes::Sieve;

use solutions::Solution;

pub fn solve() -> Solution {
    let sieve = Sieve::sieve_to(1_000_000u32);

    Solution::new(&format!("{}", sieve.nth(10001).unwrap()))
}
