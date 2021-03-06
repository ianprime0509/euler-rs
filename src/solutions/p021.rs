use math::primes::Sieve;
use solutions::Solution;

pub fn solve() -> Solution {
    let sieve = Sieve::sieve_to(100_000u32);
    let mut sum = 0;

    for n in 2..10_000 {
        // Check if n is amicable
        let d = sieve.sum_proper_divisors(n);
        if d != n && sieve.sum_proper_divisors(d) == n {
            sum += n;
        }
    }

    Solution::new(&format!("{}", sum))
}
