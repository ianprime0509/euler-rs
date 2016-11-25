use num::rational::Ratio;

use solutions::Solution;

pub fn solve() -> Solution {
    // Since we're ignoring "trivial examples", we can
    // assume that the numerator and the denominator do not
    // contain 0.
    let mut details = String::new();
    let mut prod = Ratio::new(1, 1);
    // Iterate over all possible digits in numerator and denominator
    for (n1, n2) in iproduct!(1..10, 1..10) {
        // We can restrict the denominators, knowing that n/d < 1
        for (d1, d2) in iproduct!(n1..10, 1..10) {
            let n = 10 * n1 + n2;
            let d = 10 * d1 + d2;
            // We must have n/d < 1; also, we should ignore trivial examples
            // where the numerator and denominator are both divisible by 11
            if n >= d || (n1 == n2 && d1 == d2) {
                continue;
            }
            let frac = Ratio::new(n, d);

            // Check to see if we can do any cancellations
            // Cancellation of both left digits
            if n1 == d1 {
                let ll = Ratio::new(n2, d2);
                if frac == ll {
                    details += &format!("{}/{} = {}/{}\n", n, d, n2, d2);
                    prod = prod * frac;
                }
            }

            // Cancel top left and bottom right
            if n1 == d2 {
                let lr = Ratio::new(n2, d1);
                if frac == lr {
                    details += &format!("{}/{} = {}/{}\n", n, d, n2, d1);
                    prod = prod * frac;
                }
            }

            // Cancel top right and bottom left
            if n2 == d1 {
                let rl = Ratio::new(n1, d2);
                if frac == rl {
                    details += &format!("{}/{} = {}/{}\n", n, d, n1, d2);
                    prod = prod * frac;
                }
            }

            // Cancel top left and bottom right
            if n2 == d2 {
                let ll = Ratio::new(n1, d1);
                if frac == ll {
                    details += &format!("{}/{} = {}/{}\n", n, d, n1, d1);
                    prod = prod * frac;
                }
            }
        }
    }

    Solution::with_details(&prod.denom().to_string(), &details)
}
