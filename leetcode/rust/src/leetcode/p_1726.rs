// 1726. Tuple with Same Product
// -----------------------------

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let k = nums[i] * nums[j];
                map.entry(k).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        for (_, &v) in map.iter() {
            ans += v * (v - 1) * 4;
        }
        ans
    }
}
