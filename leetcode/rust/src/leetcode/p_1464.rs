// 1464. Maximum Product of Two Elements in an Array
// -------------------------------------------------

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len() - 1;
        let m = n - 1;
        (nums[n] - 1) * (nums[m] - 1)
    }
}
