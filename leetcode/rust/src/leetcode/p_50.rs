// 50. Pow(x, n)
// -------------

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut ans = 1.0;
        let mut n = n;
        let mut x = x;
        let sign = if n < 0 { -1 } else { 1 };

        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x as f64;
        }
        if n < 0 {
            x = 1.0 / x;
        }
        while n != 0 {
            if n & 1 == 1 {
                ans = ans * x;
                if sign == -1 {
                    n += 1;
                } else {
                    n -= 1;
                }
            }
            x *= x;
            n /= 2;
        }
        ans
    }
}
