// 190. Reverse Bits
// -----------------
impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut digits = vec![];
        while n > 0 {
            digits.push(n % 2);
            n /= 2;
        }
        let mut d = 31;
        for i in 0..digits.len() {
            ans += (1 << d) * digits[i];
            d -= 1;
        }
        ans
    }
}
