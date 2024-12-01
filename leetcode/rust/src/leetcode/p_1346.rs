// 1346. Check If N and Its Double Exist
// -------------------------------------

use std::collections::HashSet;
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        for i in 0..arr.len() {
            if seen.contains(&(2 * arr[i])) || (arr[i] % 2 == 0 && seen.contains(&(arr[i] / 2))) {
                return true;
            }
            seen.insert(arr[i]);
        }
        false
    }
}
