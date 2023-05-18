// 1944. Number of Visible People in a Queue

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        let mut ans: Vec<i32> = vec![0; heights.len()];

        for i in (0..heights.len()).rev() {
            let mut current = 0;

            while stack.last().is_some() && heights[i] > *stack.last().unwrap() {
                current += 1;
                stack.pop();
            }

            if stack.len() > 0 {
                current += 1;
            }
            ans[i] = current;
            stack.push(heights[i]);
        }

        ans
    }
}
