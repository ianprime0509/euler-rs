use math::primes::Sieve;
use time;

pub fn solve() {
    let start = time::precise_time_ns();
    let limit = 10000u32;
    let sieve = Sieve::sieve_to(limit);
    println!("{}", sieve.count_divisors(28));
    let end = time::precise_time_ns();

    println!("Total time: {} ns = {} ms",
             end - start,
             ((end - start) as f64) / 1000000.0);
}
