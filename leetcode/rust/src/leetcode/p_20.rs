// 20. Valid Parentheses
// ---------------------

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if let Some(l) = stack.pop() {
                        if l != '(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if let Some(l) = stack.pop() {
                        if l != '[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some(l) = stack.pop() {
                        if l != '{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => panic!("invalid char"),
            }
        }
        if stack.len() != 0 {
            return false;
        }

        true
    }
}
