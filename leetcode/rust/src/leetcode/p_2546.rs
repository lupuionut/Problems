// 2546. Apply Bitwise Operations to Make Strings Equal
// ----------------------------------------------------

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        if s.contains('1') == false || target.contains('1') == false {
            return s == target;
        }
        true
    }
}
