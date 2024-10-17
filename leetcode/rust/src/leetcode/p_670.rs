// 670. Maximum Swap
// -----------------

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut ans = num;
        let mut digits = vec![];

        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        let mut digits = digits.into_iter().rev().collect::<Vec<_>>();

        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                let temp = digits[i];
                digits[i] = digits[j];
                digits[j] = temp;

                let mut cur = 0;
                for k in 0..digits.len() {
                    cur *= 10;
                    cur += digits[k];
                }
                ans = ans.max(cur);

                let temp = digits[i];
                digits[i] = digits[j];
                digits[j] = temp;
            }
        }

        ans
    }
}
