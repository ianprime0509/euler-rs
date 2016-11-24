use solutions::Solution;

const NAMES: &'static str = include_str!("resources/p022.txt");

pub fn solve() -> Solution {
    // Get the list of names, sorted alphabetically
    let mut names: Vec<_> = NAMES.split(',')
        .map(|s| s.replace('"', ""))
        .collect();
    names.sort();

    let mut sum = 0;
    for i in 0..names.len() {
        let score = (i as u32 + 1) * get_value(&names[i]);
        sum += score;
    }

    Solution::new(&format!("{}", sum))
}

/// Gets the alphabetical value of the word (assumed to be uppercase)
fn get_value(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| acc + (b - b'A' + 1) as u32)
}
