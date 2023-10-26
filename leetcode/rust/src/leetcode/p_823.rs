// 823. Binary Trees With Factors
// ------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut position = HashMap::new();
        let mut cache = HashMap::new();

        arr.iter().enumerate().for_each(|(k, &v)| {
            position.insert(v, k);
        });

        fn dp(
            idx: usize,
            arr: &Vec<i32>,
            position: &HashMap<i32, usize>,
            cache: &mut HashMap<usize, i64>,
        ) -> i64 {
            let mut ans = 1;
            let value = arr[idx];

            if let Some(&value) = cache.get(&idx) {
                return value;
            }

            for i in 0..idx {
                let factor1 = arr[i];
                let factor2 = value / factor1;

                if value % factor1 == 0 && position.contains_key(&factor2) {
                    ans += dp(*position.get(&factor1).unwrap(), arr, position, cache)
                        * dp(*position.get(&factor2).unwrap(), arr, position, cache);
                    ans %= 1_000_000_007;
                }
            }

            ans %= 1_000_000_007;
            cache.insert(idx, ans);
            ans
        }

        let mut ans = 0;

        for idx in 0..arr.len() {
            ans += dp(idx, &arr, &position, &mut cache);
            ans %= 1_000_000_007;
        }

        ans as i32
    }
}
