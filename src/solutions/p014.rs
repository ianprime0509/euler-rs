const LARGEST: u64 = 1_000_000;

pub fn solve() {
    let mut max = 0;
    let mut max_chain = 0;

    for n in 1..LARGEST {
        // Get the length of the Collatz sequence
        let len = collatz_len(n);
        if len > max_chain {
            max_chain = len;
            max = n;
        }
    }

    println!("{}", max);
}

fn collatz_len(n: u64) -> u32 {
    let mut n = n;
    let mut len = 1;

    while n != 1 {
        // Apply the Collatz map
        if n % 2 == 0 {
            n /= 2;
            len += 1;
        } else {
            n = (3 * n + 1) / 2;
            len += 2;
        }
    }

    len
}
