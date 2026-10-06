// 921. Minimum Add to Make Parentheses Valid
// ------------------------------------------
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = vec![];
        s.chars().for_each(|c| {
            if c == ')' {
                if let Some(&v) = stack.last() {
                    if v == '(' {
                        stack.pop();
                    } else {
                        stack.push(')');
                    }
                } else {
                    stack.push(')');
                }
            } else {
                stack.push('(');
            }
        });
        stack.len() as i32
    }
}
