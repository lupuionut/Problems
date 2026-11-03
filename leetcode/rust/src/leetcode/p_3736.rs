// 3736. Minimum Moves to Equal Array Elements III
// -----------------------------------------------

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut max = nums[0];

        for i in 1..nums.len() {
            if nums[i] > max {
                let delta = nums[i] - max;
                ans += (i as i32 * delta);
                max = nums[i];
            } else {
                ans += max - nums[i];
            }
        }

        ans
    }
}
