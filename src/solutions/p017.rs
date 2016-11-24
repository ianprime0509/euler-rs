use solutions::Solution;

pub fn solve() -> Solution {
    let mut sum = 0;
    for n in 1..1001 {
        sum += wordify(n).len();
    }
    Solution::new(&format!("{}", sum))
}

/// Turns the given number (in the range [1,1000]) into a string with no spaces or dashes
fn wordify(n: usize) -> String {
    let numbers = vec!["",
                       "one",
                       "two",
                       "three",
                       "four",
                       "five",
                       "six",
                       "seven",
                       "eight",
                       "nine",
                       "ten",
                       "eleven",
                       "twelve",
                       "thirteen",
                       "fourteen",
                       "fifteen",
                       "sixteen",
                       "seventeen",
                       "eighteen",
                       "nineteen"];
    let tens = vec!["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty",
                    "ninety"];

    // Treat 1000 as a special case (don't worry about higher numbers)
    if n == 1000 {
        return "onethousand".to_string();
    }

    let mut n = n;
    let mut s = String::new();

    // Need to account for special number names below 20
    if n % 100 < 20 {
        s += numbers[n % 100];
        n /= 100;
    } else {
        // Otherwise, the construction of numbers is regular
        s += numbers[n % 10];
        n /= 10;
        s = tens[n % 10].to_string() + &s;
        n /= 10;
    }
    if n != 0 {
        // Add "and" between hundreds and whatever comes after
        if !s.is_empty() {
            s = "and".to_string() + &s;
        }
        s = numbers[n % 10].to_string() + "hundred" + &s;
    }

    s
}
