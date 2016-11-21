pub fn solve() {
    // This problem is much more "purely mathematical" in nature than some others.
    // Process: start with the list {0, 1, ..., 9}
    // If we wish to find the Nth lexicographic permutation, note that
    // there are 9! permutations with 0 as the first character, 9! with
    // 1 as the first character, and so on.
    // Thus, the first character should be given by (N - 1) / 9! (using integer division).
    // We must remove this character in order to not reuse it.
    // Since the list will still be in order, the second character is given
    // by the element in position ((N - 1) % 9!) / 8! of the list.
    // Repeat until the list is empty.
    //
    // As done below, we can just replace N by N - 1, so that we don't have to
    // subtract 1 all the time (the N - 1 appears because of how integer division works).
    let mut list: Vec<_> = (0..10).collect();
    let mut n = 1_000_000 - 1;
    let mut result = String::new();

    for i in (0..list.len()).rev() {
        result += &list.remove(n / factorial(i)).to_string();
        n %= factorial(i);
    }

    println!("{}", result);
}

fn factorial(n: usize) -> usize {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
