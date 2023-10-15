// 1269. Number of Ways to Stay in the Same Place After Some Steps
// ---------------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();

        fn dp(idx: i32, steps: i32, end: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
            if steps == 0 {
                if idx == 0 {
                    return 1;
                }
                return 0;
            }

            if cache.contains_key(&(idx, steps)) {
                return *cache.get(&(idx, steps)).unwrap();
            }

            let mut ans: i64 = 0;
            // stay
            ans += (dp(idx, steps - 1, end, cache) % 1_000_000_007) as i64;

            if idx > 0 {
                // move back
                ans += (dp(idx - 1, steps - 1, end, cache) % 1_000_000_007) as i64;
            }

            if idx < end - 1 {
                // move front
                ans += (dp(idx + 1, steps - 1, end, cache) % 1_000_000_007) as i64;
            }

            cache.insert((idx, steps), (ans % 1_000_000_007) as i32);
            (ans % 1_000_000_007) as i32
        }

        dp(0, steps, arr_len, &mut cache)
    }
}
