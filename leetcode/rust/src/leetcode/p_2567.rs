// 2567. Minimum Score by Changing Two Elements
// --------------------------------------------

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let a = nums[n-3] - nums[0];
        let b = nums[n-2] - nums[1];
        let c = nums[n-1] - nums[2];
        if a < b {
            a.min(c)
        } else {
            b.min(c)
        }
    }
}
