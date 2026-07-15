// 3658. GCD of Odd and Even Sums
// ------------------------------
impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                return a;
            }
            if a == 0 {
                return b;
            }
            if a > b {
                return gcd(a % b, b);
            } else {
                return gcd(b % a, a);
            }
        }
        let mut a = 0;
        let mut acc = 1;
        let mut m = n;
        while m > 0 {
            a += acc;
            acc += 2;
            m -= 1;
        }
        let b = a + n;
        gcd(a, b)
    }
}
