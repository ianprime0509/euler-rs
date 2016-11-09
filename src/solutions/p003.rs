pub fn solve() {
    let mut n = 600851475143u64;
    let mut d = 1;
    // Get rid of powers of 2 first (not really necessary here but whatevs)
    if n % 2 == 0 {
        d = 2;
        n /= 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    // Now for the real stuff
    // Try odd divisors starting at 3, and get rid of all their powers before moving on
    let mut try = 3;
    while n > 1 {
        if n % try == 0 {
            d = try;
            n /= try;
            while n % try == 0 {
                n /= try;
            }
        }
        try += 2;
    }

    println!("{}", d);
}
