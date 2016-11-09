pub fn solve() {
    let (mut f1, mut f2) = (1, 1);
    let mut sum = 0u64;

    while f1 < 4000000 {
        if f1 % 2 == 0 {
            sum += f1;
        }
        let tmp = f1;
        f1 += f2;
        f2 = tmp;
    }

    println!("{}", sum);
}
