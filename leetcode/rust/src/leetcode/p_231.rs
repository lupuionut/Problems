// 231. Power of Two
// -----------------
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        for i in 0..31 {
            let m = 1 << i;
            if n ^ m == 0 {
                return true;
            }
        }
        false
    }
}
