// 1009. Complement of Base 10 Integer
// -----------------------------------
impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        let mut bits = vec![];
        let mut ans = 0;
        if n == 0 {
            return 1;
        }
        while n > 0 {
            bits.push(n % 2 ^ 1);
            n /= 2;
        }
        bits = bits.into_iter().rev().collect();

        for i in 0..bits.len() {
            ans *= 2;
            ans += bits[i];
        }
        ans
    }
}
