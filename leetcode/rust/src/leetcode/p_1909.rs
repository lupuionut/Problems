// 1909. Remove One Element to Make the Array Strictly Increasing
// --------------------------------------------------------------

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        fn can_increase(i: usize, last: i32, nums: &Vec<i32>, used: bool) -> bool {
            if i == nums.len() {
                return true;
            }
            if last < nums[i] {
                return can_increase(i + 1, nums[i], nums, used);
            }
            if used {
                return false;
            }
            let l = if i >= 2 { nums[i - 2] } else { 0 };
            can_increase(i, l, nums, true) || can_increase(i + 1, nums[i - 1], nums, true)
        }
        can_increase(1, nums[0], &nums, false)
    }
}
