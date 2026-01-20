// 3314. Construct the Minimum Bitwise Array I
// -------------------------------------------
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];
        for i in 0..nums.len() {
            for x in 0..nums[i] {
                if x | (x + 1) == nums[i] {
                    ans[i] = x;
                    break;
                }
            }
        }
        ans
    }
}
