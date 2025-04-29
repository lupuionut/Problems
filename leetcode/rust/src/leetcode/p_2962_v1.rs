// 2962. Count Subarrays Where Max Element Appears at Least K Times
// ----------------------------------------------------------------

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0i64;
        let mut max = *nums.iter().max().unwrap();
        let mut counter = 0;
        let mut l = 0;
        let mut n = nums.len();
        for r in 0..n {
            if nums[r] == max {
                counter += 1;
            }
            while l <= r && counter >= k {
                ans += 1 + (n - 1 - r) as i64;
                if nums[l] == max {
                    counter -= 1;
                }
                l += 1;
            }
        }
        ans
    }
}
