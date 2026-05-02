// 788. Rotated Digits
// -------------------
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans = 0;
        fn is_good(mut n: i32) -> bool {
            let mut digits = vec![];
            while n > 0 {
                digits.push(n % 10);
                n /= 10;
            }
            let mut ans = false;
            for i in 0..digits.len() {
                if digits[i] == 3 || digits[i] == 4 || digits[i] == 7 {
                    return false;
                }
                ans = ans || (digits[i] == 2 || digits[i] == 5 || digits[i] == 6 || digits[i] == 9);
            }
            ans
        }

        for i in 1..=n {
            if is_good(i) {
                ans += 1;
            }
        }
        ans
    }
}
