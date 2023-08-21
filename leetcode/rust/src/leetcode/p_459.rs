// 459. Repeated Substring Pattern
// -------------------------------

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let mut ans = false;
        let bytes = s.as_bytes();

        for i in 1..=n / 2 {
            let base = &bytes[0..i];
            let mut temp = true;
            let mut iter = bytes.chunks(i);
            while let Some(w) = iter.next() {
                if w != base {
                    temp = false;
                }
            }
            if temp == true {
                ans = true;
                break;
            }
        }

        ans
    }
}
