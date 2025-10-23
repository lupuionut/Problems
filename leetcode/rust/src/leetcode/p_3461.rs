// 3461. Check If Digits Are Equal in String After Operations I
// ------------------------------------------------------------
impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<usize> = s
            .chars()
            .map(|c| {
                let k = c as usize - 48;
                k
            })
            .collect();
        while digits.len() > 2 {
            for i in 0..digits.len() - 1 {
                digits[i] = (digits[i] + digits[i + 1]) % 10;
            }
            digits.pop();
        }
        digits[0] == digits[1]
    }
}
