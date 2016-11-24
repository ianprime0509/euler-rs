use solutions::Solution;

const SPIRAL_SIZE: u32 = 1001;

pub fn solve() -> Solution {
    let mut sum = 1;
    // For each diagonal, we look at odd n starting from 3
    // and going until 1001 (the nth ring) and sum the values
    // corresponding to the diagonal entry in each ring
    for k in 1..((SPIRAL_SIZE - 1) / 2 + 1) {
        // n = 2k + 1; so k goes from 1 to (n-1)/2
        let n = 2 * k + 1;
        // First diagonal: n^2 (top right)
        sum += n * n;
        // Second diagonal: n^2 - n + 1 (top left)
        sum += n * n - n + 1;
        // Third diagonal: n^2 - 2n + 2 (bottom left)
        sum += n * n - 2 * n + 2;
        // Fourth diagonal: n^2 - 3n + 3 (bottom right)
        sum += n * n - 3 * n + 3;
    }

    // At this point, it's pretty clear that we don't even
    // need a program to do this:
    // ANSWER = 1 + \sum_{k=1}^{(s-1)/2} (4(2k+1)^2 - 6(2k+1) + 6)
    // where s is the spiral size.
    //
    // Using the formula for the sum of consecutive integers/squares
    // to derive an explicit closed form is left as an exercise to the
    // reader :^)

    Solution::new(&format!("{}", sum))
}
