// 1015. Smallest Integer Divisible by K
// -------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut curr = 1;
        let mut count = 1;
        let mut used = HashSet::new();

        while curr % k != 0 {
            curr *= 10;
            curr += 1;
            count += 1;

            curr %= k;
            if !used.insert(curr) {
                return -1;
            }
        }
        count
    }
}
