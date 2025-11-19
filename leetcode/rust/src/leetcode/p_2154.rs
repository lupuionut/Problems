// 2154. Keep Multiplying Found Values by Two
// ------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        let mut ans = original;
        loop {
            if !set.contains(&ans) {
                break;
            }
            ans <<= 1;
        }
        ans
    }
}
