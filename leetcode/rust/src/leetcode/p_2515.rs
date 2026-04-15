// 2515. Shortest Distance to Target String in a Circular Array
// ------------------------------------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut existing: HashMap<String, Vec<i32>> = HashMap::new();
        let n = words.len() as i32;
        words.into_iter().enumerate().for_each(|(k, v)| {
            existing
                .entry(v)
                .and_modify(|mut vals| {
                    vals.push(k as i32);
                })
                .or_insert(vec![k as i32]);
        });
        if let Some(idxs) = existing.get(&target) {
            let mut best = i32::MAX;
            for &idx in idxs {
                if idx == start_index {
                    best = 0;
                    break;
                }
                if idx > start_index {
                    let d = (idx - start_index).min(start_index + n - idx);
                    best = best.min(d);
                } else {
                    let d = (start_index - idx).min(n - start_index + idx);
                    best = best.min(d);
                }
            }
            return best;
        } else {
            return -1;
        }
    }
}
