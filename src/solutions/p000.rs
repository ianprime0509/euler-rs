use math::primes::Sieve;
use time;
use num::{BigUint, FromPrimitive};

pub fn solve() {
    let start = time::precise_time_ns();
    let limit: u32 = FromPrimitive::from_u32(100).unwrap();
    let sieve = Sieve::sieve_to(&limit).unwrap();
    println!("{}", sieve.count_divisors(&28));
    let end = time::precise_time_ns();

    println!("Total time: {} ns = {} ms",
             end - start,
             ((end - start) as f64) / 1000000.0);
}
