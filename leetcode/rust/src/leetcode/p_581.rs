// 581. Shortest Unsorted Continuous Subarray
// ------------------------------------------

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        fn keep_len(nums: &Vec<i32>) -> usize {
            let mut stack = vec![];
            let mut started = false;
            for i in 0..nums.len() {
                if stack.len() > 0 && nums[i] < *stack.last().unwrap() {
                    started = true;
                }
                if started {
                    while stack.len() > 0 && nums[i] < *stack.last().unwrap() {
                        stack.pop();
                    }
                } else {
                    stack.push(nums[i]);
                }
            }
            stack.len()
        }

        let prefix = keep_len(&nums);
        if prefix == nums.len() {
            return 0;
        }
        let mut nums = nums.into_iter().rev().map(|n| -n).collect::<Vec<_>>();
        let suffix = keep_len(&nums);
        (nums.len() - prefix - suffix) as i32
    }
}
