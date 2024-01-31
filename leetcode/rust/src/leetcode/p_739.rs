// 739. Daily Temperatures
// -----------------------

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(usize, i32)> = vec![];
        let mut ans = vec![0; temperatures.len()];

        for i in (0..temperatures.len()).rev() {
            while stack.len() > 0 && stack.last().unwrap().1 <= temperatures[i] {
                stack.pop();
            }
            if stack.len() > 0 {
                ans[i] = (stack.last().unwrap().0 - i) as i32;
            }
            stack.push((i, temperatures[i]));
        }

        ans
    }
}
