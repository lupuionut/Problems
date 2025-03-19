// Minimum Operations to Make Binary Array Elements Equal to One I
// ---------------------------------------------------------------

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        for i in 0..n - 2 {
            if nums[i] == 0 {
                ans += 1;
                nums[i + 1] = (nums[i + 1] - 1).abs();
                nums[i + 2] = (nums[i + 2] - 1).abs();
            }
        }
        if nums[n - 1] == 0 || nums[n - 2] == 0 {
            return -1;
        }
        ans
    }
}
