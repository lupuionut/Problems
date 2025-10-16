// 2598. Smallest Missing Non-negative Integer After Operations
// ------------------------------------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i].rem_euclid(value);
            counter.entry(n).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut i = 0;
        loop {
            let s = i % value;

            if let Some(mut v) = counter.get_mut(&s) {
                if *v == 0 {
                    return i;
                } else {
                    *v -= 1;
                }
            } else {
                return i;
            }
            i += 1;
        }
        -1
    }
}
