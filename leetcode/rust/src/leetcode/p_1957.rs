// 1957. Delete Characters to Make Fancy String
// --------------------------------------------

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut stack = vec![];
        let mut s = s.as_bytes();

        s.iter().for_each(|&c| {
            let n = stack.len();
            if n < 2 {
                stack.push(c);
            } else {
                if &stack[n - 2..n] != &[c, c] {
                    stack.push(c);
                }
            }
        });

        String::from_utf8(stack).unwrap()
    }
}
