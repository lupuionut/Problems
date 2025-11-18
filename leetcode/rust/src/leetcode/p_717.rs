// 717. 1-bit and 2-bit Characters
// -------------------------------
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut stack = 0;
        for i in 0..bits.len() - 1 {
            if stack == 1 {
                stack = 0;
            } else if bits[i] == 1 {
                stack = 1;
            }
        }
        if stack == 1 {
            return false;
        }
        true
    }
}
