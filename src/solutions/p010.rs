use math::primes::Sieve;

use solutions::Solution;

pub fn solve() -> Solution {
    let sieve = Sieve::sieve_to(2000000u64);
    let sum = sieve.primes().iter().fold(0, |acc, p| acc + p);

    Solution::new(&format!("{}", sum))
}
