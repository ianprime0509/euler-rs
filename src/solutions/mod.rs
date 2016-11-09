mod p000;
mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;

pub fn solve(n: i32) {
    match n {
        0 => p000::solve(),
        1 => p001::solve(),
        2 => p002::solve(),
        3 => p003::solve(),
        4 => p004::solve(),
        5 => p005::solve(),
        6 => p006::solve(),
        7 => p007::solve(),
        _ => println!("unimplemented"),
    }
}
