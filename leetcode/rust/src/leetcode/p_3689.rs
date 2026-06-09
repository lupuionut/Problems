// 3689. Maximum Total Subarray Value I
// ------------------------------------
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut a = i32::MAX;
        let mut b = i32::MIN;

        for i in 0..nums.len() {
            a = a.min(nums[i]);
            b = b.max(nums[i]);
        }

        (b - a) as i64 * k as i64
    }
}
