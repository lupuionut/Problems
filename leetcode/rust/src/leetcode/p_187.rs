// 187. Repeated DNA Sequences
// ---------------------------

use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut s = s.as_str();
        let mut seq: HashMap<&str, i32> = HashMap::new();
        let mut ans: Vec<String> = Vec::new();

        if s.len() < 10 {
            return ans;
        }

        for i in 0..=s.len() - 10 {
            let key = &s[i..i + 10];
            match seq.get(&key) {
                Some(val) => seq.insert(key, *val + 1),
                None => seq.insert(key, 1),
            };
        }

        for (key, val) in seq.iter() {
            if *val > 1 {
                ans.push(key.to_string());
            }
        }

        ans
    }
}
