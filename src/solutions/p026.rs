pub fn solve() {
    // ( ͡° ͜ʖ ͡°)
    let mut longest_d = 0;
    for d in 1..1000 {
        if cycle_len(d) > longest_d {
            longest_d = d;
        }
    }

    println!("{}", longest_d);
}

/// Returns the length of the recurring cycle in 1/n (0 if there is none)
fn cycle_len(n: u32) -> u32 {
    // Algorithm:
    // 1/n = (1/10)(10/n); let 10/n = q + r/n for q, r integers and r < n.
    // Then q gives the next digit in the decimal expansion of 1/n, and
    // r is the next numerator: using Euclidean division on 10r/n
    // gives the next digit, and so on...
    // Cycle detection here is rather poor (keeping a list of all numerators
    // encountered so far and halting when one reoccurs) and could
    // probably be improved upon.

    // Keep track of which numerators we've encountered so far
    let mut nums = Vec::new();
    let mut r = 1;
    for i in 0.. {
        // If the expansion terminates, just get out and return 0
        if r == 0 {
            break;
        }

        // Look for a cycle
        if let Some(j) = nums.iter().position(|&m| r == m) {
            return i - j as u32;
        }

        // Proceed to next step
        nums.push(r);
        r = 10 * r % n;
    }

    0
}
