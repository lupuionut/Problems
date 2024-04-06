// 1249. Minimum Remove to Make Valid Parentheses
// ----------------------------------------------

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut valid: Vec<char> = vec![];
        let mut close = vec![0; s.len()];
        let mut open = 0;

        s.chars().enumerate().for_each(|(i, c)| {
            let curr = if c == ')' { 1 } else { 0 };
            if i > 0 {
                close[i] = close[i - 1] + curr;
            } else {
                close[i] = curr;
            }
        });

        s.chars().enumerate().for_each(|(i, c)| match c {
            '(' => {
                let n = s.len() - 1;
                let remain = close[n] - close[i];
                if remain > open {
                    open += 1;
                    valid.push(c);
                }
            }
            ')' => {
                if open > 0 {
                    valid.push(c);
                    open -= 1;
                }
            }
            _ => {
                valid.push(c);
            }
        });

        valid.into_iter().collect::<String>()
    }
}
