use math::primes::Sieve;
use time;

pub fn solve() {
    let start = time::precise_time_ns();
    let sieve = Sieve::sieve_to(1000).unwrap();
    println!("{}", sieve.is_prime(&472_882_047));
    let end = time::precise_time_ns();

    println!("Total time: {} ns = {} ms",
             end - start,
             ((end - start) as f64) / 1000000.0);
}
