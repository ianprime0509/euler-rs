pub fn solve() {
    // If we search for larger triplets first, we should get to the answer quicker
    for c in (1..1000).rev() {
        for b in 1..c {
            if b * b >= c * c {
                break;
            }

            for a in 1..b {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }

    println!("Something is wrong");
}
