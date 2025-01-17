// 2683. Neighboring Bitwise XOR
// -----------------------------

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut ans = 0;
        derived.iter().for_each(|n| {
            ans ^= n;
        });
        if ans == 0 {
            return true;
        }
        false
    }
}
