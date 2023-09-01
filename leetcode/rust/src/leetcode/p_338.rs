// 338. Counting Bits
// ------------------

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut i = 1;
        let n = n as usize;
        let mut ans = vec![0; (n + 1)];

        while i <= n {
            ans[i] = ans[i / 2] + i as i32 % 2;
            i += 1;
        }

        ans
    }
}
