// 880. Decoded String at Index
// ----------------------------

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut total_expanded: i64 = s.as_bytes().iter().fold(0, |acc, &b| {
            if b.is_ascii_digit() {
                acc * ((b as i64) - 48)
            } else {
                acc + 1
            }
        });

        let mut k = k as i64;
        let mut ans = "".to_string();

        for b in s.chars().rev() {
            k %= total_expanded;
            if b.is_ascii_alphabetic() {
                if k == 0 {
                    return String::from(b);
                } else {
                    total_expanded -= 1;
                }
            } else {
                total_expanded /= b.to_digit(10).unwrap() as i64;
            }
        }

        ans
    }
}
