// 2864. Maximum Odd Binary Number
// -------------------------------

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut ones = 0;
        let mut zeroes = 0;

        s.chars().for_each(|c| {
            if c == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        });

        let mut s = if ones > 1 {
            "1".repeat(ones - 1)
        } else {
            "".to_string()
        };
        s.push_str("0".repeat(zeroes).as_str());
        if ones > 0 {
            s.push_str("1");
        }
        s
    }
}
