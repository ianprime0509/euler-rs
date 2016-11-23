use math::primes::Sieve;

pub fn solve() {
    let sieve = Sieve::sieve_to(&1_000_000u32).unwrap();

    // You don't have to square any numbers, just use divided differences
    let mut triangle = 1u32;
    let mut inc = 2;
    loop {
        let sigma = sieve.count_divisors(&triangle);
        if sigma > 500 {
            println!("{}", triangle);
            return;
        }

        triangle += inc;
        inc += 1;
    }
}
