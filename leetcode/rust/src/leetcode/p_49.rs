// 49. Group Anagrams
// ------------------
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        strs.iter().for_each(|s| {
            let mut key = vec![0; 26];
            s.chars().for_each(|c| {
                let k = c as usize - 97;
                key[k] += 1;
            });
            anagrams
                .entry(key)
                .and_modify(|v| v.push(s.to_string()))
                .or_insert(vec![s.to_string()]);
        });

        anagrams
            .into_iter()
            .map(|(k, v)| v)
            .collect::<Vec<Vec<String>>>()
    }
}
