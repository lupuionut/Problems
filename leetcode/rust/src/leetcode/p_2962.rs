// 2962. Count Subarrays Where Max Element Appears at Least K Times
// ----------------------------------------------------------------

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0;
        let mut left = 0;
        let mut count = 0;
        let needle = *nums.iter().max().unwrap();

        for right in 0..nums.len() {
            if nums[right] == needle {
                count += 1;
            }
            if count >= k {
                let total_after = (nums.len() - right - 1) as i64;
                while count >= k {
                    ans += (1 + total_after);
                    if nums[left] == needle {
                        count -= 1;
                    }
                    left += 1;
                }
            }
        }

        ans
    }
}
