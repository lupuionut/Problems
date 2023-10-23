// 342. Power of Four
// ------------------

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        // power of two
        if n & (n - 1) == 0 {
            // power of four
            if n as i64 & 0xaaaaaaaa == 0 {
                return true;
            }
        }
        false
    }
}
