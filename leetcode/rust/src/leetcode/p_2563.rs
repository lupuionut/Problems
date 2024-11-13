// 2563. Count the Number of Fair Pairs
// ------------------------------------

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut left_idx = nums.partition_point(|&x| x < lower - nums[i]);
            let mut right_idx = nums.partition_point(|&x| x <= upper - nums[i]);
            if right_idx > i {
                right_idx = i;
            }
            ans += 0.max((right_idx - left_idx) as i64);
        }

        ans
    }
}
