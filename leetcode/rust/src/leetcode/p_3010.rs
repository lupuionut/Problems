// 3010. Divide an Array Into Subarrays With Minimum Cost I
// --------------------------------------------------------
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut nxt = (i32::MAX, i32::MAX);
        for i in 1..nums.len() {
            if nums[i] < nxt.0 {
                nxt.1 = nxt.0;
                nxt.0 = nums[i];
            } else if nums[i] < nxt.1 {
                nxt.1 = nums[i];
            }
        }
        nums[0] + nxt.0 + nxt.1
    }
}
