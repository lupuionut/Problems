// 673. Number of Longest Increasing Subsequence
// ---------------------------------------------

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![1, 1]; nums.len()];
        let mut longest = 1;
        let mut ans = 0;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[i][0] < dp[j][0] + 1 {
                        dp[i][0] = dp[j][0] + 1;
                        dp[i][1] = dp[j][1];
                        longest = longest.max(dp[i][0]);
                    } else if dp[i][0] == dp[j][0] + 1 {
                        dp[i][1] += dp[j][1];
                    }
                }
            }
        }

        dp.iter().for_each(|x| {
            if x[0] == longest {
                ans += x[1];
            }
        });
        ans
    }
}
