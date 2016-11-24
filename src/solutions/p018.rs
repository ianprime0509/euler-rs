use solutions::Solution;

const TRI: &'static str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

pub fn solve() -> Solution {
    // Get triangle array
    let mut tri: Vec<Vec<u32>> = TRI.lines()
        .map(|l| {
            l.trim()
                .split(' ')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    // Here is what to do to find the maximum path:
    // Start at the second-to-last row:
    // For each element in this row, add to it the maximum of
    // the two elements below it in the triangle; repeat this process
    // for rows, working up to the top row.
    // The value in the top row after iterating this process will be
    // the maximum path sum.
    for i in (0..(tri.len() - 1)).rev() {
        for j in 0..tri[i].len() {
            tri[i][j] += max(tri[i + 1][j], tri[i + 1][j + 1]);
        }
    }

    Solution::new(&format!("{}", tri[0][0]))
}

fn max<T: Ord + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
