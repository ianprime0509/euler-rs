use std::string::ToString;

use solutions::Solution;

pub fn solve() -> Solution {
    // Try products of 3 digit numbers
    let mut largest = 0;
    for n in (100..999).rev() {
        // Assume WLOG that n >= m
        for m in (100..n).rev() {
            let prod = n * m;
            if prod > largest && is_palindrome(prod) {
                largest = prod;
            }
        }
    }

    Solution::new(&format!("{}", largest))
}

// Doesn't account for Unicode because it doesn't matter for this problem
fn is_palindrome<T: ToString>(x: T) -> bool {
    let str = x.to_string();
    let str = str.as_bytes();

    for i in 0..(str.len() / 2) {
        if str[i] != str[str.len() - i - 1] {
            return false;
        }
    }

    true
}
