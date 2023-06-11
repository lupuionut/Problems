// 628. Maximum Product of Three Numbers
// -------------------------------------

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let l = nums.len();
        nums.sort();

        if nums[0] < 0 && nums[1] < 0 {
            if nums[0] * nums[1] * nums[l - 1] > nums[l - 1] * nums[l - 2] * nums[l - 3] {
                return nums[0] * nums[1] * nums[l - 1];
            }
        }
        nums[l - 1] * nums[l - 2] * nums[l - 3]
    }
}
