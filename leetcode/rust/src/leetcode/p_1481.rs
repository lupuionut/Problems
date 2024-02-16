// 1481. Least Number of Unique Integers after K Removals
// ------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut freqs: HashMap<i32, i32> = HashMap::new();

        arr.iter().for_each(|&n| {
            num_freq.entry(n).and_modify(|c| *c += 1).or_insert(1);
        });
        let unique = num_freq.len() as i32;

        for (_, v) in num_freq {
            freqs.entry(v).and_modify(|c| *c += 1).or_insert(1);
        }
        // (freq, count of numbers with same freq)
        let mut freqs = freqs.into_iter().collect::<Vec<(i32, i32)>>();
        freqs.sort_by(|a, b| b.0.cmp(&a.0));

        let mut removed = 0;
        while k > 0 && freqs.len() > 0 {
            let (freq, count_freq) = freqs.pop().unwrap();
            if freq > k {
                break;
            }
            let y = count_freq.min(k / freq);
            k -= (y * freq);
            removed += y;
        }

        unique - removed
    }
}
