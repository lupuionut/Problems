// 3536. Maximum Product of Two Digits
// -----------------------------------
impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut f = 0;
        let mut s = 0;
        while n > 0 {
            let d = n % 10;
            n /= 10;
            if d > f {
                s = f;
                f = d;
            } else if d > s {
                s = d;
            }
        }
        f * s
    }
}
