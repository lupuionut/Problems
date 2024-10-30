// 1671. Minimum Number of Removals to Make Mountain Array
// -------------------------------------------------------

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut ans = i32::MAX;

        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    left[i] = left[i].max(left[j] + 1);
                }
            }
        }

        for i in (0..n).rev() {
            for j in (i..n).rev() {
                if nums[i] > nums[j] {
                    right[i] = right[i].max(right[j] + 1);
                }
            }
        }

        for i in 0..n {
            if left[i] > 1 && right[i] > 1 {
                ans = ans.min(n as i32 - (left[i] + right[i] - 1));
            }
        }
        ans
    }
}
