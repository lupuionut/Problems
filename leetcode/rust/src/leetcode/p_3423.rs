// 3423. Maximum Difference Between Adjacent Elements in a Circular Array
// ----------------------------------------------------------------------

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = i32::MIN;
        for i in 0..n {
            ans = ans.max((nums[i] - nums[(i + 1) % n]).abs());
        }
        ans
    }
}
