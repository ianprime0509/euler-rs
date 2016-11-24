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
mod p014;
mod p015;
mod p016;
mod p017;
mod p018;
mod p019;
mod p020;
mod p021;
mod p022;
mod p023;
mod p024;
mod p025;
mod p026;
mod p027;
mod p028;
mod p029;
mod p030;
mod p031;
mod p032;

use std::fmt;

#[derive(Debug)]
pub struct Solution {
    answer: String,
    details: String,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.details.is_empty() {
            write!(f, "ANSWER: {}", self.answer)
        } else {
            write!(f, "{}\nANSWER: {}", self.details, self.answer)
        }
    }
}

impl Solution {
    pub fn new(answer: &str) -> Solution {
        Solution::with_details(answer, "")
    }

    pub fn with_details(answer: &str, details: &str) -> Solution {
        Solution {
            answer: answer.to_owned(),
            details: details.to_owned(),
        }
    }

    /// Gets the answer contained within this `Solution`; this should be
    /// exactly the text that gets entered into the solution box
    /// on the Project Euler website
    pub fn answer(&self) -> String {
        self.answer.clone()
    }
}

pub fn solve(n: i32) -> Solution {
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
        14 => p014::solve(),
        15 => p015::solve(),
        16 => p016::solve(),
        17 => p017::solve(),
        18 => p018::solve(),
        19 => p019::solve(),
        20 => p020::solve(),
        21 => p021::solve(),
        22 => p022::solve(),
        23 => p023::solve(),
        24 => p024::solve(),
        25 => p025::solve(),
        26 => p026::solve(),
        27 => p027::solve(),
        28 => p028::solve(),
        29 => p029::solve(),
        30 => p030::solve(),
        31 => p031::solve(),
        32 => p032::solve(),
        _ => Solution::new("unimplemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    macro_rules! test {
        ($n:ident, $i:expr, $s:expr) => {
            #[test]
            fn $n() {
                if $s != solve($i).answer() {
                    panic!("Expected: '{}' Got: '{}'", $s, solve($i).answer());
                }
            }
        }
    }

    test!(p001, 1, "233168");
    test!(p002, 2, "4613732");
    test!(p003, 3, "6857");
    test!(p004, 4, "906609");
    test!(p005, 5, "232792560");
    test!(p006, 6, "25164150");
    test!(p007, 7, "104743");
    test!(p008, 8, "23514624000");
    test!(p009, 9, "31875000");
    test!(p010, 10, "142913828922");
    test!(p011, 11, "70600674");
    test!(p012, 12, "76576500");
    // TODO: add rest of tests
}
