// 3174. Clear Digits
// ------------------

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut c = vec![];
        s.as_bytes().into_iter().for_each(|&b| {
            if b >= 48 && b <= 57 {
                if c.len() > 0 {
                    c.pop();
                }
            } else {
                c.push(b);
            }
        });
        String::from_utf8(c).unwrap()
    }
}
