// 2710. Remove Trailing Zeros From a String
// -----------------------------------------

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut bytes = num.as_bytes();
        let mut buffer = vec![];
        let mut ans = vec![];

        for i in 0..bytes.len() {
            if bytes[i] == b'0' {
                buffer.push(bytes[i]);
            } else {
                if buffer.len() > 0 && ans.len() > 0 {
                    ans.append(&mut buffer);
                    buffer = vec![];
                };
                ans.push(bytes[i]);
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
