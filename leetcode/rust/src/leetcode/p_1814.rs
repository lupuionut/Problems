// 1814. Count Nice Pairs in an Array
// ----------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        fn reverse(n: i32) -> i32 {
            let mut number = 0;
            let mut from = n;
            while from > 0 {
                number = number * 10 + from % 10;
                from /= 10;
            }
            number
        }

        let mut total = 0;
        let mut pairs: HashMap<i32, i32> = HashMap::new();

        nums.iter().for_each(|&n| {
            let diff = n - reverse(n);
            if let Some(&val) = pairs.get(&diff) {
                total += val;
                total %= 1_000_000_007;
            }
            pairs
                .entry(diff)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        });

        total % 1_000_000_007
    }
}
