// 326. Power of Three

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let max: i32 = 1162261467;
        return n >= 1 && max % n == 0;
    }
}
