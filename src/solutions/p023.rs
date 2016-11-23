use math::primes::Sieve;

pub fn solve() {
    // Get a list of all the abundant numbers less than 28123
    let sieve = Sieve::sieve_to(&28123u32).unwrap();
    let abundant: Vec<_> = (2..28123)
        .filter(|&n| sieve.sum_proper_divisors(&n) > n)
        .collect();

    // Now try to find integers which cannot be written as the sum of two
    // abundant numbers
    let mut sum = 0;
    'outer: for n in 1..28124 {
        // Try n - a for all abundant numbers a and test for abundancy
        for &a in &abundant {
            // We can stop searching if n - a <= 0
            if a >= n {
                break;
            }
            // Test n - a for abundancy (we know the abundant list is in order)
            if let Ok(_) = abundant.binary_search(&(n - a)) {
                // n - a is abundant, so n is the sum of two abundant numbers
                continue 'outer;
            }
        }

        // We have exhausted all possibilities, so n cannot be written
        // as such a sum
        sum += n;
    }

    println!("{}", sum);
}
