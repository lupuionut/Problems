// 2799. Count Complete Subarrays in an Array
// ------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut curr = HashMap::new();
        nums.iter().for_each(|x| {
            curr.entry(x).and_modify(|c| *c = 0).or_insert(0);
        });
        let mut l = 0;
        let mut ans = 0;
        for r in 0..nums.len() {
            curr.entry(&nums[r]).and_modify(|c| *c += 1).or_insert(1);
            'lo: loop {
                if l > r {
                    break;
                }
                for &v in curr.values() {
                    if v == 0 {
                        break 'lo;
                    }
                }
                if let Some(_) = curr.get(&nums[l]) {
                    ans += (nums.len() - r) as i32;
                    curr.entry(&nums[l]).and_modify(|c| *c -= 1).or_insert(0);
                    l += 1;
                }
            }
        }
        ans
    }
}
