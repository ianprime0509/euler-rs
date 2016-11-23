use math::primes::Sieve;
use math::numeric;
use time;
use num::{Float, FromPrimitive, BigUint};

pub fn solve() {
    let start = time::precise_time_ns();
    let limit: u32 = FromPrimitive::from_u32(100).unwrap();
    let sieve = Sieve::sieve_to(&limit).unwrap();
    println!("{}", sieve.count_divisors(&28));
    let end = time::precise_time_ns();

    println!("Total time: {} ns = {} ms",
             end - start,
             ((end - start) as f64) / 1000000.0);

    let stuff: BigUint = FromPrimitive::from_u32(2).unwrap();
    println!("{}", numeric::pow_primint(&stuff, 1000u32));
}
