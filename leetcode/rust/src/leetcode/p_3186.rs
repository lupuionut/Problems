// 3186. Maximum Total Damage With Spell Casting
// ---------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        let mut counter = HashMap::new();
        for i in 0..power.len() {
            counter.entry(power[i]).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut keys = counter.keys().map(|&v| v as i32).collect::<Vec<i32>>();
        keys.sort();
        let mut cache = vec![-1i64; keys.len()];

        fn dp(i: usize, keys: &Vec<i32>, cache: &mut Vec<i64>, counter: &HashMap<i32, i32>) -> i64 {
            if i >= keys.len() {
                return 0;
            }

            if cache[i] != -1 {
                return cache[i];
            }

            let mut j = i + 1;
            while j < keys.len() && keys[j] <= keys[i] + 2 {
                j += 1;
            }

            let mut best = dp(i + 1, keys, cache, counter);
            if let Some(&c) = counter.get(&keys[i]) {
                let value = c as i64 * keys[i] as i64;
                best = best.max(value + dp(j, keys, cache, counter));
            }

            cache[i] = best;
            best
        }

        dp(0, &keys, &mut cache, &counter)
    }
}
