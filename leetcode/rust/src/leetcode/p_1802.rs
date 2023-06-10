// 1802. Maximum Value at a Given Index in a Bounded Array
// ------------------------------------------------------
// binary search starting from 1 to max_sum and compare left side sum with right side sum

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let left_values: u64 = index as u64;
        let right_values: u64 = (n - index - 1) as u64;
        let mut high = max_sum;
        let mut low = 1;
        let mut ans = 0;

        while low <= high {
            let middle: u64 = ((high - low) / 2 + low) as u64;
            let mut sum: u64 = middle as u64;
            let mut left_sum: u64 = 0;
            let mut right_sum: u64 = 0;
            let m: u64 = (middle - 1) as u64;

            // in case we need to fill right side with less numbers than middle
            // m = 4 and in left we need 3 2 1 or 3 2
            if right_values <= m {
                right_sum = (m * (m + 1) / 2) - ((m - right_values) * (m - right_values + 1) / 2);
            // in case we need to fill the right side with more values, but we will go to 1
            // for ex: m = 4, and we have 6 spots in right, 3 2 1 and we fill the rest with 1 1 1 so it's just
            // the difference between how many places we have in right and the value for m
            } else {
                right_sum = (m * (m + 1) / 2) + (right_values - m);
            }

            if left_values <= m {
                left_sum = (m * (m + 1) / 2) - ((m - left_values) * (m - left_values + 1) / 2);
            } else {
                left_sum = (m * (m + 1) / 2) + (left_values - m);
            }

            sum += left_sum + right_sum;

            if sum <= max_sum as u64 {
                low = (middle + 1) as i32;
                ans = middle;
            } else {
                high = (middle - 1) as i32;
            }
        }

        ans as i32
    }
}
