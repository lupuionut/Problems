// 1547. Minimum Cost to Cut a Stick
// ---------------------------------

use core::i32::MAX;
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();

        fn dp(left: i32, right: i32, cuts: &Vec<i32>, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
            if left + 1 >= right {
                return 0;
            }
            if cache.contains_key(&(left, right)) {
                return *cache.get(&(left, right)).unwrap();
            }
            let mut best = MAX;
            for cut in cuts {
                if left < *cut && *cut < right {
                    let cost =
                        (right - left) + dp(left, *cut, cuts, cache) + dp(*cut, right, cuts, cache);
                    best = best.min(cost);
                }
            }
            if best == MAX {
                cache.insert((left, right), 0);
                return 0;
            } else {
                cache.insert((left, right), best);
                return best;
            }
        }
        dp(0, n, &cuts, &mut cache)
    }
}
