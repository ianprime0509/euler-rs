use std::fmt;

macro_rules! setup_modules {
    ($solve:ident : $($name:ident, $num:expr);+) => {
        $(
            mod $name;
        )*

        pub fn $solve(n: i32) -> Solution {
            match n {
                $(
                    $num => $name::solve(),
                )*
                _ => Solution::new("undefined"),
            }
        }
    };
}

setup_modules!(solve :
               p000, 0;
               p001, 1;
               p002, 2;
               p003, 3;
               p004, 4;
               p005, 5;
               p006, 6;
               p007, 7;
               p008, 8;
               p009, 9;
               p010, 10;
               p011, 11;
               p012, 12;
               p013, 13;
               p014, 14;
               p015, 15;
               p016, 16;
               p017, 17;
               p018, 18;
               p019, 19;
               p020, 20;
               p021, 21;
               p022, 22;
               p023, 23;
               p024, 24;
               p025, 25;
               p026, 26;
               p027, 27;
               p028, 28;
               p029, 29;
               p030, 30;
               p031, 31;
               p032, 32;
               p033, 33;
               p034, 34;
               p035, 35
);

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
    test!(p013, 13, "5537376230");
    test!(p014, 14, "837799");
    test!(p015, 15, "137846528820");
    test!(p016, 16, "1366");
    test!(p017, 17, "21124");
    test!(p018, 18, "1074");
    test!(p019, 19, "171");
    test!(p020, 20, "648");
    test!(p021, 21, "31626");
    test!(p022, 22, "871198282");
    test!(p023, 23, "4179871");
    test!(p024, 24, "2783915460");
    test!(p025, 25, "4782");
    test!(p026, 26, "983");
    test!(p027, 27, "-59231");
    test!(p028, 28, "669171001");
    test!(p029, 29, "9183");
    test!(p030, 30, "443839");
    test!(p031, 31, "73682");
    test!(p032, 32, "45228");
    test!(p033, 33, "100");
    test!(p034, 34, "40730");
    test!(p035, 35, "55");
}
