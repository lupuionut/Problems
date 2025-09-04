// 3516. Find Closest Person
// -------------------------

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let f = (x - z).abs();
        let s = (y - z).abs();
        if f == s {
            return 0;
        } else if f < s {
            return 1;
        } else {
            return 2;
        }
    }
}
