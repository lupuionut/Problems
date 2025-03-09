// 3208. Alternating Groups II
// ---------------------------

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];
        let mut head = 0;
        let mut tail = 0;

        while tail < colors.len() {
            if let Some(last) = stack.pop() {
                if last == colors[head % colors.len()] {
                    stack.clear();
                } else {
                    stack.push(last);
                }
            }
            stack.push(colors[head % colors.len()]);

            if (head as i32 - tail as i32 + 1).abs() == k {
                if stack.len() >= k as usize {
                    ans += 1;
                }
                tail += 1;
            }
            head += 1;
        }

        ans
    }
}
