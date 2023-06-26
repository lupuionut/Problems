// 2680. Maximum OR
// ----------------

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut prefix: Vec<i64> = vec![0; n + 1];
        let mut suffix: Vec<i64> = vec![0; n + 1];
        let mut ans = 0;

        for i in 0..n {
            prefix[i + 1] = prefix[i] | nums[i] as i64;
        }

        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] | nums[i] as i64;
        }

        for i in 0..n {
            let current = ((nums[i] as i64) << (k as i64)) | prefix[i] | suffix[i + 1];
            ans = ans.max(current);
        }

        ans
    }
}
