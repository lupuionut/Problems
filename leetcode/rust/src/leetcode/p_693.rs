// 693. Binary Number with Alternating Bits
// ----------------------------------------
impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut last = None;
        while n > 0 {
            let curr = n % 2;
            n /= 2;
            if let Some(v) = last {
                if v == curr {
                    return false;
                }
            }
            last = Some(curr);
        }
        true
    }
}
