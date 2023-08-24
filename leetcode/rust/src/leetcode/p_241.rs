// 241. Different Ways to Add Parentheses
// --------------------------------------
// catalan numbers

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Value(i32),
    Operation(u8),
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let tokens = Solution::parse(expression.as_bytes());
        Solution::evaluate(0, tokens.len() - 1, &tokens)
    }

    fn evaluate(left: usize, right: usize, tokens: &Vec<Token>) -> Vec<i32> {
        if left == right {
            if let Token::Value(val) = tokens[left] {
                return vec![val];
            }
        }

        let mut i = left + 1;
        let mut ans = vec![];

        while i < right {
            let left_ans = Solution::evaluate(left, i - 1, tokens);
            let right_ans = Solution::evaluate(i + 1, right, tokens);

            for l in &left_ans {
                for r in &right_ans {
                    match tokens[i] {
                        Token::Operation(43) => ans.push(l + r),
                        Token::Operation(45) => ans.push(l - r),
                        Token::Operation(42) => ans.push(l * r),
                        _ => {}
                    }
                }
            }
            i += 2;
        }

        ans
    }

    fn parse(s: &[u8]) -> Vec<Token> {
        let mut ans = vec![];
        let mut current = vec![];

        s.iter().for_each(|&c| {
            if c.is_ascii_digit() {
                current.push(c);
            } else {
                if current.len() > 0 {
                    let st = String::from_utf8(current.clone()).unwrap();
                    let n = st.parse::<i32>().unwrap();
                    ans.push(Token::Value(n));
                }
                current.clear();
                ans.push(Token::Operation(c));
            }
        });

        if current.len() > 0 {
            let st = String::from_utf8(current.clone()).unwrap();
            let n = st.parse::<i32>().unwrap();
            ans.push(Token::Value(n));
        }

        ans
    }
}
