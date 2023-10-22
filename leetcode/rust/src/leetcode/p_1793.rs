// 1793. Maximum Score of a Good Subarray
// --------------------------------------

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = nums[k];
        let mut curr_min = nums[k];
        let mut i = k;
        let mut j = k;

        while i > 0 || j < nums.len() - 1 {
            if i == 0 || (j < nums.len() - 1 && nums[j + 1] > nums[i - 1]) {
                j += 1;
                curr_min = curr_min.min(nums[j]);
            } else {
                i -= 1;
                curr_min = curr_min.min(nums[i]);
            }
            ans = ans.max(curr_min * (j - i + 1) as i32);
        }

        ans
    }
}
