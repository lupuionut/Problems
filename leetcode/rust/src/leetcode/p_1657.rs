// 1657. Determine if Two Strings Are Close
// ----------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut c_w1 = vec![0; 26];
        let mut c_w2 = vec![0; 26];

        word1.chars().for_each(|c| {
            let key = (c as usize) - 97;
            c_w1[key] += 1;
        });

        word2.chars().for_each(|c| {
            let key = (c as usize) - 97;
            c_w2[key] += 1;
        });

        let mut f_w1 = HashMap::new();
        let mut f_w2 = HashMap::new();

        // count how many freq are in each word
        for i in 0..26 {
            let freq0 = c_w1[i];
            if freq0 != 0 {
                f_w1.entry(freq0)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            let freq1 = c_w2[i];
            if freq1 != 0 {
                f_w2.entry(freq1)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
            // if we have a letter that doesn't exist in the other word
            if (freq0 == 0 && freq1 != 0) || (freq1 == 0 && freq0 != 0) {
                return false;
            }
        }

        for (key, val) in f_w1.iter() {
            if let Some(val2) = f_w2.get(&key) {
                if val2 != val {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
