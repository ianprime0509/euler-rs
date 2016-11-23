use math::primes::Sieve;

pub fn solve() {
    let sieve = Sieve::sieve_to(1_000_000u32);

    println!("{}", sieve.nth(10001).unwrap());
}
