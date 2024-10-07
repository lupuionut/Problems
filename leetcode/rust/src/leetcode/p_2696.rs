// 2696. Minimum String Length After Removing Substrings
// -----------------------------------------------------

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = vec![];
        s.chars().for_each(|c| {
            let last = stack.last();
            match last {
                Some(l) => {
                    if (*l == 'A' && c == 'B') || (*l == 'C' && c == 'D') {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                },
                None => {
                    stack.push(c);
                }
            }
        });
        
        stack.len() as i32
    }
}
