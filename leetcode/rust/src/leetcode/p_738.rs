// 738. Monotone Increasing Digits
// -------------------------------

impl Solution {
    pub fn monotone_increasing_digits(mut n: i32) -> i32 {
        if n < 10 {
            return n;
        }

        let mut digits = vec![];
        while n > 0 {
            let d = n % 10;
            n /= 10;
            digits.push(d);
        }
        let mut digits = digits.into_iter().collect::<Vec<_>>();
        let mut found = true;

        while found {
            found = false;
            for i in (1..digits.len()).rev() {
                if found {
                    digits[i - 1] = 9;
                }
                if digits[i] > digits[i - 1] {
                    digits[i] = if digits[i] > 0 { digits[i] - 1 } else { 9 };
                    digits[i - 1] = 9;
                    found = true;
                }
            }
        }

        let mut ans = *digits.last().unwrap();
        digits.pop();
        while digits.len() > 0 {
            let last = digits.pop().unwrap();
            ans = ((ans * 10) + last);
        }

        ans
    }
}
