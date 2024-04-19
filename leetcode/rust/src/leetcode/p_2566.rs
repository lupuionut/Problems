// 2566. Maximum Difference by Remapping a Digit
// ---------------------------------------------

impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let digits = Solution::into_digits(num);

        let mut replacement = None;
        let mx = digits
            .iter()
            .rev()
            .map(|&d| {
                if let Some(value) = replacement {
                    if d == value {
                        9
                    } else {
                        d
                    }
                } else {
                    if d < 9 {
                        replacement = Some(d);
                    }
                    9
                }
            })
            .collect::<Vec<_>>();

        replacement = None;
        let mn = digits
            .iter()
            .map(|&d| {
                if let Some(value) = replacement {
                    if d == value {
                        0
                    } else {
                        d
                    }
                } else {
                    if d > 0 {
                        replacement = Some(d);
                    }
                    0
                }
            })
            .rev()
            .collect::<Vec<_>>();

        let mx = Solution::into_num(mx);
        let mn = Solution::into_num(mn);
        mx - mn
    }

    fn into_digits(mut n: i32) -> Vec<i32> {
        let mut ans = vec![];
        while n > 0 {
            let d = n % 10;
            ans.push(d);
            n /= 10;
        }
        ans
    }

    fn into_num(digits: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..digits.len() {
            ans = ans * 10 + digits[i];
        }
        ans
    }
}
