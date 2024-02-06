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
            let to_insert = if let Some(words) = anagrams.get(&key) {
                let mut words = words.clone();
                words.push(s.to_string());
                words
            } else {
                vec![s.to_string()]
            };
            anagrams.insert(key, to_insert);
        });

        anagrams
            .iter()
            .map(|(k, v)| v.clone())
            .collect::<Vec<Vec<String>>>()
    }
}
