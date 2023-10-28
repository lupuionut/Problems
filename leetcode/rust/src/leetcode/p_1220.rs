// 1220. Count Vowels Permutation
// ------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        fn construct(i: i32, last: char, cache: &mut HashMap<(i32, char), i64>) -> i64 {
            if i == 0 {
                return 1;
            }

            if let Some(&value) = cache.get(&(i, last)) {
                return value;
            }

            let mut ways = 0;
            if last == 'a' {
                ways += (construct(i - 1, 'e', cache) % 1_000_000_007);
            } else if last == 'e' {
                ways += (construct(i - 1, 'a', cache) % 1_000_000_007);
                ways += (construct(i - 1, 'i', cache) % 1_000_000_007);
            } else if last == 'i' {
                ways += (construct(i - 1, 'a', cache) % 1_000_000_007);
                ways += (construct(i - 1, 'e', cache) % 1_000_000_007);
                ways += (construct(i - 1, 'o', cache) % 1_000_000_007);
                ways += (construct(i - 1, 'u', cache) % 1_000_000_007);
            } else if last == 'o' {
                ways += (construct(i - 1, 'i', cache) % 1_000_000_007);
                ways += (construct(i - 1, 'u', cache) % 1_000_000_007);
            } else {
                ways += (construct(i - 1, 'a', cache) % 1_000_000_007);
            }

            cache.insert((i, last), ways % 1_000_000_007);
            ways % 1_000_000_007
        }

        let mut ans = 0;
        let mut cache: HashMap<(i32, char), i64> = HashMap::new();

        for &c in &['a', 'e', 'i', 'o', 'u'] {
            ans += (construct(n - 1, c, &mut cache) % 1_000_000_007);
        }

        (ans % 1_000_000_007) as i32
    }
}
