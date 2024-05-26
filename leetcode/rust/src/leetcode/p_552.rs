// 552. Student Attendance Record II
// ---------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        fn count(n: i32, a: i32, l: i32, cache: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
            if n == 0 {
                return 1;
            }

            if let Some(&val) = cache.get(&(n, a, l)) {
                return val;
            }

            let mut total = 0;

            // present
            total += count(n - 1, a, 0, cache);
            total %= 1_000_000_007;

            // absent
            if a == 0 {
                total += count(n - 1, a + 1, 0, cache);
                total %= 1_000_000_007;
            }

            // late
            if l < 2 {
                total += count(n - 1, a, l + 1, cache);
                total %= 1_000_000_007;
            }

            let val = total % 1_000_000_007;
            cache.insert((n, a, l), val);
            val
        }

        let mut cache = HashMap::new();
        count(n, 0, 0, &mut cache)
    }
}
