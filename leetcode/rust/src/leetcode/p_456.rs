// 456. 132 Pattern
// ----------------

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        // stack (j - biggest, i - smallest)
        let mut stack: Vec<(i32, i32)> = vec![];
        let mut min = nums[0];

        for i in 1..nums.len() {
            while stack.len() > 0 {
                let last = stack.last().unwrap();
                if nums[i] >= last.0 {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() > 0 {
                let last = stack.last().unwrap();
                if nums[i] > last.1 {
                    return true;
                }
            }
            stack.push((nums[i], min));
            min = min.min(nums[i]);
        }

        false
    }
}
