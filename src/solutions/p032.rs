use std::collections::HashSet;

pub fn solve() {
    // Useful facts:
    // 99*99 = 9801 < 12345
    // 1111*11 = 12221 > 789
    // 111*111 = 12321 > 789
    // In other words, we only need to check expressions
    // where the multiplicand/multiplier have 2 and 3 or 1 and 4 digits,
    // respectively (if we enforce the condition that the
    // first term always has fewer digits than the second, we
    // can avoid some duplicates)

    // This seems like an exceptionally bad way of doing this
    let mut products = HashSet::new();
    let mut digits = HashSet::new();
    for (d1, d2, d3, d4, d5) in iproduct!(1..10, 1..10, 1..10, 1..10, 1..10) {
        // Don't forget to clear the set before doing this stuff
        digits.clear();

        // Insert the digits (since it's a hash set, we won't get duplicates)
        digits.insert(d1);
        digits.insert(d2);
        digits.insert(d3);
        digits.insert(d4);
        digits.insert(d5);
        if digits.len() != 5 {
            // Get out if some digits coincide
            continue;
        }
        // Insert 0 so we don't have to worry about edge cases
        digits.insert(0);

        // First case: 2 and 3 digits
        let mut digits23 = digits.clone();
        // Check to see if the product is pandigital
        let n1 = 10 * d1 + d2;
        let n2 = 100 * d3 + 10 * d4 + d5;
        let prod = n1 * n2;
        let mut tmp = prod;
        // Get the 4 digits from prod
        for _ in 0..4 {
            digits23.insert(tmp % 10);
            tmp /= 10;
        }

        // If the product was 4 digits and we've used every digit from
        // 0 to 9 (including the 0 we inserted earlier), then we have
        // a pandigital product
        if tmp == 0 && digits23.len() == 10 {
            println!("{} x {} = {}", n1, n2, prod);
            products.insert(prod);
        }

        // Second case: 1 and 4 digits
        let mut digits14 = digits.clone();
        // Same stuff as above
        let n1 = d1;
        let n2 = 1000 * d2 + 100 * d3 + 10 * d4 + d5;
        let prod = n1 * n2;
        let mut tmp = prod;
        for _ in 0..4 {
            digits14.insert(tmp % 10);
            tmp /= 10;
        }
        if tmp == 0 && digits14.len() == 10 {
            println!("{} x {} = {}", n1, n2, prod);
            products.insert(prod);
        }
    }

    println!("{}", products.iter().fold(0, |acc, x| acc + x));
}
