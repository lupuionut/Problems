// 1493. Longest Subarray of 1's After Deleting One Element
// --------------------------------------------------------

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut compressed = vec![];
        let mut streak = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if streak > 0 {
                    compressed.push(streak);
                    streak = 0;
                }
                compressed.push(0);
            } else {
                streak += nums[i];
            }
        }
        if streak > 0 {
            compressed.push(streak);
        }

        if compressed.len() == 1 {
            if compressed[0] == 0 {
                return 0;
            } else {
                return compressed[0] - 1;
            }
        }

        if compressed.len() == 2 || compressed.len() == 3 {
            return compressed.iter().sum();
        }

        let mut ans = 0;
        let mut total = 0;
        for i in 0..compressed.len() {
            total += compressed[i];
            if i > 2 {
                total -= compressed[i - 3];
            }
            ans = ans.max(total);
        }
        ans
    }
}
