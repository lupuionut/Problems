// 966. Vowel Spellchecker
// -----------------------

use std::collections::HashMap;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        fn vowel_replace(s: &String) -> String {
            let mut n = String::new();
            for c in s.chars() {
                match c {
                    'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => n.push('*'),
                    _ => n.push(c.to_ascii_lowercase()),
                }
            }
            n
        }

        let mut ans = vec![];
        let mut dict = HashMap::new();
        let mut vowels = HashMap::new();
        let mut caps = HashMap::new();
        for i in 0..wordlist.len() {
            dict.insert(wordlist[i].clone(), i);
            let v = vowel_replace(&wordlist[i]);
            let s = wordlist[i].as_str().to_ascii_lowercase();
            if !vowels.contains_key(&v) {
                vowels.insert(v.clone(), i);
            }
            if !caps.contains_key(&s) {
                caps.insert(s.clone(), i);
            }
        }

        for i in 0..queries.len() {
            if let Some(&k) = dict.get(&queries[i]) {
                ans.push(wordlist[k].clone());
                continue;
            }

            let s = queries[i].as_str().to_ascii_lowercase();
            if let Some(&k) = caps.get(&s) {
                ans.push(wordlist[k].clone());
                continue;
            }

            let v = vowel_replace(&queries[i]);
            if let Some(&k) = vowels.get(&v) {
                ans.push(wordlist[k].clone());
                continue;
            }

            ans.push("".to_string());
        }
        ans
    }
}
