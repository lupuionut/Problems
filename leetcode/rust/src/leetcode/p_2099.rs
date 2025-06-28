// 2099. Find Subsequence of Length K With the Largest Sum
// -------------------------------------------------------

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums
            .iter()
            .enumerate()
            .map(|(k, &v)| (k, v))
            .collect::<Vec<(usize, i32)>>();
        nums.sort_by(|a, b| b.1.cmp(&a.1));
        let mut nums = nums[0..k as usize].to_vec();
        nums.sort_by(|a, b| a.0.cmp(&b.0));
        nums.iter().map(|a| a.1).collect()
    }
}
