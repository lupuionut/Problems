// 1021. Remove Outermost Parentheses
// ----------------------------------
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut depth = 0;
        s.chars().for_each(|c| {
            if c == '(' {
                depth += 1;
            } 
            if depth > 1 {
                stack.push(c);
            } 
            if c == ')' {
                depth -= 1;
            }
        });

        stack.iter().collect::<String>()
    }
}
