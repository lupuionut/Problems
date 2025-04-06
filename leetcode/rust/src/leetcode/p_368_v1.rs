// 368. Largest Divisible Subset
// -----------------------------

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut dp = vec![1; nums.len()];
        let mut prev = vec![-1; nums.len()];
        let mut ans = vec![];

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        prev[i] = j as i32;
                    }
                }
            }
        }

        let best = *dp.iter().max().unwrap();

        for i in (0..dp.len()).rev() {
            if dp[i] == best {
                let mut curr = i;
                while prev[curr] != -1 {
                    ans.push(nums[curr]);
                    curr = prev[curr] as usize;
                }
                ans.push(nums[curr]);
                break;
            }
        }

        ans.into_iter().rev().collect()
    }
}
