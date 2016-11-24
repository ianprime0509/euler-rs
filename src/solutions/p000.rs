use math::primes::Sieve;
use solutions::Solution;

use time;

pub fn solve() -> Solution {
    let start = time::precise_time_ns();
    let sieve = Sieve::sieve_to(100u32);
    let answer = format!("{:?}", sieve.primes());
    let end = time::precise_time_ns();

    let details = format!("Total time: {} ns = {} ms",
                          end - start,
                          ((end - start) as f64) / 1000000.0);

    Solution::with_details(&answer, &details)
}
