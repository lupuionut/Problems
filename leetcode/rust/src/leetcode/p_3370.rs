// 3370. Smallest Number With All Set Bits
// ---------------------------------------
impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut m = 1;
        while m < n {
            m <<= 1;
            m |= 1;
        }
        m
    }
}
