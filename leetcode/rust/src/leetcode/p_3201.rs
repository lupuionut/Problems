// 3201. Find the Maximum Length of Valid Subsequence I
// ----------------------------------------------------

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut odd = 0;
        let mut even = 0;
        let mut end_odd = 0;
        let mut end_even = 0;

        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
                end_odd = end_odd.max(1 + end_even);
                even += 1;
            } else {
                end_even = end_even.max(1 + end_odd);
                odd += 1;
            }
        }
        ans = ans.max(odd);
        ans = ans.max(even);
        ans = ans.max(end_odd);
        ans = ans.max(end_even);

        ans
    }
}
