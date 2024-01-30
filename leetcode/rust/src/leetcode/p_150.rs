// 150. Evaluate Reverse Polish Notation
// -------------------------------------

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        fn eval(i: i32, tokens: &Vec<String>) -> (i32, i32) {
            let token = &tokens[i as usize];
            match token.as_str() {
                "+" => {
                    let (a, next) = eval(i - 1, tokens);
                    let (b, next) = eval(next, tokens);
                    (a + b, next)
                }
                "-" => {
                    let (a, next) = eval(i - 1, tokens);
                    let (b, next) = eval(next, tokens);
                    (b - a, next)
                }
                "*" => {
                    let (a, next) = eval(i - 1, tokens);
                    let (b, next) = eval(next, tokens);
                    (a * b, next)
                }
                "/" => {
                    let (a, next) = eval(i - 1, tokens);
                    let (b, next) = eval(next, tokens);
                    (b / a, next)
                }
                _ => {
                    return (token.parse::<i32>().unwrap(), i - 1);
                }
            }
        }

        let n = tokens.len() - 1;
        eval(n as i32, &tokens).0
    }
}
