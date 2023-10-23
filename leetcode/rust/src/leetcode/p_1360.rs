// 1360. Number of Days Between Two Dates
// --------------------------------------

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let mut ans = 0;
        let parts1 = date1
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let parts2 = date2
            .split("-")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut date1 = parts1.clone();
        let mut date2 = parts2.clone();

        // find the starting date (date1) and ending date (date2)
        if parts1[0] == parts2[0] {
            if parts1[1] == parts2[1] {
                if parts1[2] == parts2[2] {
                    return 0;
                } else if parts1[2] > parts2[2] {
                    date1 = parts2.clone();
                    date2 = parts1.clone();
                }
            } else if parts2[1] < parts1[1] {
                date1 = parts2.clone();
                date2 = parts1.clone();
            }
        } else if parts2[0] < parts1[0] {
            date1 = parts2.clone();
            date2 = parts1.clone();
        }

        // if we handle different years, get the remaining days from the year
        // by substracting from 365(6) the days already passed
        if date1[0] != date2[0] {
            let mut days = &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            if Solution::is_leap(date1[0]) {
                ans += 366;
                days = &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            } else {
                ans += 365;
            }
            // substract days from months until current month
            let mut month: usize = 0;
            while (month as i32) < date1[1] - 1 {
                ans -= days[month];
                month += 1;
            }

            // substract number of days
            ans -= date1[2];

            // iterate over all years until target year and add corresponding days
            let mut year = date1[0] + 1;
            while year < date2[0] {
                if Solution::is_leap(year) {
                    ans += 366;
                } else {
                    ans += 365;
                }
                year += 1;
            }
        }

        let mut month = if date1[0] == date2[0] {
            date1[1] - 1
        } else {
            0
        };

        let mut days = if Solution::is_leap(date2[0]) {
            &[31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            &[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };

        // add the days corresponding to months passed in target year
        while month < date2[1] - 1 {
            ans += days[month as usize];
            month += 1;
        }

        // add the days from the current month of target year
        ans += date2[2];
        if date1[0] == date2[0] {
            ans -= date1[2];
        }

        ans
    }

    fn is_leap(year: i32) -> bool {
        if year % 4 == 0 {
            if year % 100 == 0 && year % 400 != 0 {
                return false;
            }
            return true;
        }
        false
    }
}
