// 2873. Maximum Value of an Ordered Triplet I
// -------------------------------------------

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut prefix = nums.clone();
        let mut suffix = nums.clone();

        for i in 1..nums.len() {
            prefix[i] = prefix[i - 1].max(prefix[i]);
        }

        for i in (1..nums.len()).rev() {
            suffix[i - 1] = suffix[i - 1].max(suffix[i]);
        }

        for j in 1..nums.len() - 1 {
            let t = (prefix[j - 1] - nums[j]) as i64 * suffix[j + 1] as i64;
            ans = ans.max(t);
        }
        ans
    }
}
