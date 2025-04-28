// 2302. Count Subarrays With Score Less Than K
// --------------------------------------------

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut ans = 0;
        let mut l = 0;
        let mut curr = 0i64;
        for r in 0..nums.len() {
            curr += nums[r] as i64;
            while l <= r && (curr * (r - l + 1) as i64) - k >= 0 {
                curr -= nums[l] as i64;
                l += 1;
            }
            ans += (r - l + 1) as i64;
        }

        ans
    }
}
