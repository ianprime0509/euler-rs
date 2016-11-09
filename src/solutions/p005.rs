use num::Integer;

pub fn solve() {
    let mut lcm: u64 = 2;
    for n in 3..21 {
        lcm = lcm.lcm(&n);
    }

    println!("{}", lcm);
}
