// 2283. Check if Number Has Equal Digit Count and Digit Value

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut freq = vec![0; 10];

        for value in num.chars() {
            let digit = (value as i32) - 48;
            freq[digit as usize] += 1;
        }

        for (i, value) in num.chars().enumerate() {
            let expect = (value as i32) - 48;
            if freq[i] != expect {
                return false;
            }
        }

        true
    }
}
