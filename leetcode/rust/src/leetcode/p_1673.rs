// 1673. Find the Most Competitive Subsequence
// -------------------------------------------

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = vec![];

        for i in 0..nums.len() {
            while stack.len() > 0
                && stack.last().is_some()
                && stack.len() + nums.len() - i > k as usize
            {
                if *stack.last().unwrap() <= nums[i] {
                    break;
                }
                stack.pop();
            }
            if stack.len() < k as usize {
                stack.push(nums[i]);
            }
        }

        stack
    }
}
