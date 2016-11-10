mod p000;
mod p001;
mod p002;
mod p003;
mod p004;
mod p005;
mod p006;
mod p007;
mod p008;
mod p009;
mod p010;
mod p011;
mod p012;
mod p013;

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
        8 => p008::solve(),
        9 => p009::solve(),
        10 => p010::solve(),
        11 => p011::solve(),
        12 => p012::solve(),
        13 => p013::solve(),
        _ => println!("unimplemented"),
    }
}
