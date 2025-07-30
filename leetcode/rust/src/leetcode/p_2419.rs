// 2419. Longest Subarray With Maximum Bitwise AND
// -----------------------------------------------

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mx = *nums.iter().max().unwrap();

        let mut l = 0;
        let mut curr = nums[0];
        for r in 0..nums.len() {
            if curr & nums[r] == mx {
                ans = ans.max((r - l) as i32 + 1);
            } else {
                curr = nums[r];
                l = r;
            }
        }
        if curr == mx {
            ans = ans.max((nums.len() - l) as i32);
        }

        ans
    }
}
