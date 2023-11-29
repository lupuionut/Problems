// 191. Number of 1 Bits
// ---------------------

impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut ans = 0;
        while n != 0 {
            if n & 1 == 1 {
                ans += 1;
            }
            n = n >> 1;
        }

        ans
    }
}
