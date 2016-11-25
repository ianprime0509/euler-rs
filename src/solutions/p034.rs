use solutions::Solution;

pub fn solve() -> Solution {
    // We only need to check numbers up to (and including) 6 digits
    let mut details = String::new();
    let mut sum = 0;
    for n in 10..10000000 {
        if n == digit_fact(n) {
            details += &format!("{}\n", n);
            sum += n;
        }
    }

    Solution::with_details(&sum.to_string(), &details)
}

/// Computes the digit factorial
fn digit_fact(n: u32) -> u32 {
    let mut ans = 0;
    let mut n = n;
    while n > 0 {
        ans += fact(n % 10);
        n /= 10;
    }
    ans
}

/// Factorial (only for digits)
fn fact(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        _ => 0,
    }
}
