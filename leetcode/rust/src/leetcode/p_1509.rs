// 1509. Minimum Difference Between Largest and Smallest Value in Three Moves
// --------------------------------------------------------------------------

impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        if n <= 4 {
            return 0;
        }
        let mut options = vec![];
        options.push(nums[n - 3] - nums[1]);
        options.push(nums[n - 2] - nums[2]);
        options.push(nums[n - 1] - nums[3]);
        options.push(nums[n - 4] - nums[0]);

        *options.iter().min().unwrap()
    }
}
