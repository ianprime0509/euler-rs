const COIN_VALS: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

pub fn solve() {
    // We can do this by using a "restricted partition" function:
    // Start with whatever amount (in this case 200p) and
    // then calculate the number of partitions involving coins recursively.
    println!("{}", partition_coins(200, 200));
}

/// Returns the number of partitions of `amt` into coins of
/// value less than or equal to `max_coin`.
fn partition_coins(amt: i32, max_coin: i32) -> u32 {
    // To avoid counting some partitions more than once,
    // we limit the maximum denomination to be used in each partition.
    // In other words, we construct the partition in descending order
    // of coin value.
    if amt < 0 {
        0
    } else if amt == 0 {
        1
    } else {
        // Add number of partitions recursively such that we
        // always construct the partition in descending order of
        // denomination.
        COIN_VALS.iter()
            .filter(|&&v| v <= max_coin)
            .fold(0, |acc, &v| acc + partition_coins(amt - v, v))
    }
}
