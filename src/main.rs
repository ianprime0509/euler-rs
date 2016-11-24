extern crate bit_vec;
#[macro_use]
extern crate itertools;
extern crate num;
extern crate time;

use std::io::{self, BufRead, Write};

mod solutions;
mod math;

fn main() {
    print!("Problem number: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();

    println!("{}", solutions::solve(n));
}
