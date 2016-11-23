use math::primes::Sieve;

use num::Signed;

pub fn solve() {
    let sieve = Sieve::sieve_to(&1_000_000u64).unwrap();

    let (mut max_a, mut max_b, mut max_n) = (0, 0, 0);
    for a in -1000..1000i64 {
        // We can ignore b = 1000, because 1000 isn't prime
        for b in -1000..1000i64 {
            // Use divided differences to calculate polynomial values
            // (n+1)^2 + a(n+1) + b - (n^2 + an + b) = 2n + 1 + a
            let mut p = b;
            let mut inc = 1 + a;
            for n in 0.. {
                if !sieve.is_prime(&(p.abs() as u64)) {
                    // At this point, n is the number of successive primes
                    if n > max_n {
                        max_a = a;
                        max_b = b;
                        max_n = n;
                    }
                    break;
                }

                // Get next polynomial value
                p += inc;
                inc += 2;
            }
        }
    }

    println!("{}", max_a * max_b);
}
