// 2707. Extra Characters in a String
// ----------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut dict = HashMap::new();
        let mut s = s.as_bytes();
        let mut cache = vec![-1; s.len()];

        fn dfs(i: usize, s: &[u8], dict: &HashMap<u8, Vec<String>>, cache: &mut Vec<i32>) -> i32 {
            if i == s.len() {
                return 0;
            }

            if cache[i] != -1 {
                return cache[i];
            }

            let mut ans = s.len() as i32;
            let key = s[i];
            let remain = s.len() - i;

            if dict.contains_key(&key) {
                let words = dict.get(&key).unwrap();
                for word in words.iter() {
                    let n = word.len();
                    if n <= remain {
                        let wb = word.as_bytes();
                        if wb == &s[i..(i + n)] {
                            ans = ans.min(dfs(i + n, s, dict, cache));
                        }
                    }
                }
            }

            ans = ans.min(1 + dfs(i + 1, s, dict, cache));
            cache[i] = ans;
            ans
        }

        for i in 0..dictionary.len() {
            let w = dictionary[i].as_bytes();
            dict.entry(w[0])
                .and_modify(|e: &mut Vec<String>| e.push(dictionary[i].clone()))
                .or_insert(vec![dictionary[i].clone()]);
        }

        dfs(0, s, &dict, &mut cache)
    }
}
