pub fn solve() {
    // By observing that 6 * 9^5 < 999999, it is clear that
    // we don't have to check numbers with more than 6 digits
    // (and by the problem statement, we shouldn't check 1 digit
    // numbers either)
    let mut sum = 0;
    for n in 10..1_000_000 {
        if n == sum_digit_fifth_powers(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn sum_digit_fifth_powers(n: u32) -> u32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        sum += d * d * d * d * d;
    }

    sum
}
