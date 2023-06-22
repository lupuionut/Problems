// 1913. Maximum Product Difference Between Two Pairs
// --------------------------------------------------

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len() - 1;
        nums.sort();

        (nums[n] * nums[n - 1]) - (nums[0] * nums[1])
    }
}
