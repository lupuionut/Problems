// 2873. Maximum Value of an Ordered Triplet I
// -------------------------------------------

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    ans = ans.max((nums[i] - nums[j]) as i64 * nums[k] as i64);
                }
            }
        }

        ans
    }
}
