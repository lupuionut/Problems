// 2520. Count the Digits That Divide a Number

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut target = num;
        let mut ans = 0;
        loop {
            if target == 0 {
                break;
            }
            let n = target % 10;
            if num % n == 0 {
                ans += 1;
            }
            target /= 10;
        }
        ans
    }
}
