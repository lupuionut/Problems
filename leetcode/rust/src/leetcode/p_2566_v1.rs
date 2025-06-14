// 2566. Maximum Difference by Remapping a Digit
// ---------------------------------------------

impl Solution {
    pub fn min_max_difference(mut num: i32) -> i32 {
        let mut digits = vec![];
        let mut diff = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        let n = digits.len();
        let mut r_min = digits[n - 1];
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

            if min == r_min {
                min = 0;
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
