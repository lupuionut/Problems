// 3754. Concatenate Non-Zero Digits and Multiply by Sum I
// -------------------------------------------------------
impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut sum = 0i64;
        let mut x = 0i64;
        let mut mult = 1i64;
        while n > 0 {
            let d = (n % 10) as i64;
            if d != 0 {
                sum += d;
                x += (d * mult);
                mult *= 10;
            }
            n /= 10;
        }
        x * sum
    }
}
