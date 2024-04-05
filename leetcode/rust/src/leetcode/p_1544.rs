// 1544. Make The String Great
// ---------------------------

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        s.chars().for_each(|c| {
            if stack.len() == 0 {
                stack.push(c);
            } else {
                let last = *stack.last().unwrap() as i32;
                let curr = c as i32;
                if (last - curr).abs() == 32 {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        });

        stack.into_iter().collect::<String>()
    }
}
