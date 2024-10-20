// 1106. Parsing A Boolean Expression
// ----------------------------------

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack = vec![];
        expression.chars().for_each(|c| match c {
            ')' => {
                let mut substack = vec![];
                while stack.len() > 0 {
                    let last = stack.pop().unwrap();
                    if last == '(' {
                        let operation = stack.pop().unwrap();
                        let val = substack
                            .iter()
                            .map(|&e| if e == 't' { true } else { false })
                            .reduce(|acc, e| {
                                if operation == '&' {
                                    acc & e
                                } else if operation == '|' {
                                    acc | e
                                } else {
                                    e
                                }
                            });

                        if let Some(v) = val {
                            match (v, operation) {
                                (true, '!') => stack.push('f'),
                                (false, '!') => stack.push('t'),
                                (true, _) => stack.push('t'),
                                (false, _) => stack.push('f'),
                                _ => {}
                            }
                        }
                        break;
                    } else {
                        substack.push(last);
                    }
                }
            }
            ',' => {}
            _ => {
                stack.push(c);
            }
        });
        if stack[0] == 't' {
            return true;
        } else {
            return false;
        }
    }
}
