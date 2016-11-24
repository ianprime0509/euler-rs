use solutions::Solution;

pub fn solve() -> Solution {
    // 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    let mut day_of_week = 1;
    // Loop through years and months and count when the first of the month
    // lies on Sunday
    let mut first_sundays = 0;
    // The following loop advances the day of the week month by month
    // First, advance through the year 1900 to get to January, 1901
    for month in 1..13 {
        day_of_week = (day_of_week + days_in_month(month, 1900)) % 7;
    }
    for year in 1901..2001 {
        for month in 1..13 {
            // Add one to the counter if the first day of the month is a Sunday
            if day_of_week == 0 {
                first_sundays += 1;
            }
            day_of_week = (day_of_week + days_in_month(month, year)) % 7;
        }
    }

    Solution::new(&format!("{}", first_sundays))
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(month: u32, year: u32) -> u32 {
    match month {
        9 | 4 | 6 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}
