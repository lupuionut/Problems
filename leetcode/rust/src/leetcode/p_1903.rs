// 1903. Largest Odd Number in String
// ----------------------------------

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let mut end = Some(0);
        for (i, b) in num.as_bytes().iter().enumerate() {
            if b & 1 == 1 {
                end = Some(i + 1);
            }
        }
        num[0..end.unwrap()].to_string()
    }
}
