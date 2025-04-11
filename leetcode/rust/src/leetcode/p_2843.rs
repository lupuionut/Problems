// 2843. Count Symmetric Integers
// ------------------------------

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut ans = 0;
        for mut n in low..=high {
            let mut digits = vec![];
            while n > 0 {
                digits.push(n % 10);
                n /= 10;
            }
            let l = digits.len();
            if l % 2 != 0 {
                continue;
            }
            if &digits[0..l / 2].iter().sum::<i32>() == &digits[l / 2..].iter().sum::<i32>() {
                ans += 1;
            }
        }
        ans
    }
}
