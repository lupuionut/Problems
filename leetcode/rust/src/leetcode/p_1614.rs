// 1614. Maximum Nesting Depth of the Parentheses
// ----------------------------------------------

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut best = 0;
        let mut open = 0;

        s.chars().for_each(|c| match c {
            '(' => {
                open += 1;
            }
            ')' => {
                best = best.max(open);
                if open > 0 {
                    open -= 1;
                }
            }
            _ => {}
        });

        best
    }
}
