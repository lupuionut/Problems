// 1432. Max Difference You Can Get From Changing an Integer
// ---------------------------------------------------------

impl Solution {
    pub fn max_diff(mut num: i32) -> i32 {
        let mut digits = vec![];
        let mut diff = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        let n = digits.len();
        let mut r_min = -1;
        let mut mn = -1;

        // if all digits the same
        if digits.iter().all(|&x| x == digits[n - 1]) {
            mn = 1;
            r_min = digits[n - 1];
        } else {
            for i in (0..n).rev() {
                if r_min == -1 && digits[i] > 1 {
                    if i == n - 1 {
                        mn = 1;
                    } else {
                        mn = 0;
                    }
                    r_min = digits[i];
                    break;
                }
            }
            if r_min == -1 {
                mn = 1;
            }
        }

        for i in (0..n).rev() {
            if r_min == -1 && digits[i] > 1 {
                r_min = digits[i];
            }
        }

        let mut r_max = -1;
        for i in (0..n).rev() {
            let mut max = digits[i];
            let mut min = digits[i];
            if r_max == -1 {
                if digits[i] < 9 {
                    r_max = digits[i];
                    max = 9;
                }
            } else {
                if digits[i] == r_max {
                    max = 9;
                }
            }

            if digits[i] == r_min {
                min = mn;
            }
            diff.push(max - min);
        }

        let mut ans = 0;
        for i in 0..diff.len() {
            ans *= 10;
            ans += diff[i];
        }
        ans
    }
}
