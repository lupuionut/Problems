// 2733. Neither Minimum nor Maximum
// ---------------------------------

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return -1;
        }
        let mx = nums[0].max(nums[1]);
        let mn = nums[0].min(nums[1]);
        if nums[2] > mx {
            return mx;
        }
        if nums[2] < mn {
            return mn;
        }
        nums[2]
    }
}
