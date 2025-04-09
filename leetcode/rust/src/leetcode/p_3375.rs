// 3375. Minimum Operations to Make Array Values Equal to K
// --------------------------------------------------------

use std::collections::HashSet;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: HashSet<i32> = HashSet::from_iter(nums);
        let mut n = 0;
        for &v in &set {
            if v < k {
                return -1;
            }
            n += 1;
        }
        if set.contains(&k) {
            n - 1
        } else {
            n
        }
    }
}
