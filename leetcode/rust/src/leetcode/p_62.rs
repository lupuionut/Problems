// 62. Unique Paths
// ----------------

use std::collections::HashMap;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut cache = HashMap::new();

        fn dfs(position: (i32, i32), m: i32, n: i32, cache: &mut HashMap<(i32, i32), i32>) -> i32 {
            if position.0 == m - 1 && position.1 == n - 1 {
                return 1;
            }

            if let Some(&val) = cache.get(&position) {
                return val;
            }

            let mut ans = 0;
            if position.0 < m - 1 {
                ans += dfs((position.0 + 1, position.1), m, n, cache);
            }

            if position.1 < n - 1 {
                ans += dfs((position.0, position.1 + 1), m, n, cache);
            }

            if !cache.contains_key(&position) {
                cache.insert(position, ans);
            }

            ans
        }

        dfs((0, 0), m, n, &mut cache)
    }
}
