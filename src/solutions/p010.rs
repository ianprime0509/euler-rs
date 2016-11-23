use math::primes::Sieve;

pub fn solve() {
    let sieve = Sieve::sieve_to(2_000_000u64);
    let sum = sieve.primes().iter().fold(0, |acc, p| acc + p);

    println!("{}", sum);
}
