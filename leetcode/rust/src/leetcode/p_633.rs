// 633. Sum of Square Numbers
// --------------------------

use std::collections::HashSet;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut i = 0i64;
        let mut set = HashSet::new();

        loop {
            let j = i * i;
            if j > c as i64 {
                break;
            }
            set.insert(j);
            i += 1;
        }

        for a in set.iter() {
            let b = c as i64 - a;
            if set.contains(&b) {
                return true;
            }
        }

        false
    }
}
