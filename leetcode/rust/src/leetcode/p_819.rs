// 819. Most Common Word
// ---------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let mut paragraph = paragraph;
        let mut b = HashSet::new();
        let mut f = HashMap::new();
        let mut ans = ("".to_string(), 0);

        for w in banned {
            b.insert(w);
        }

        for p in &['!', '?', '\'', ',', ';', '.'] {
            paragraph = paragraph.replace(*p, " ");
        }
        paragraph.make_ascii_lowercase();
        let words = paragraph.split(" ").collect::<Vec<_>>();

        for word in words {
            f.entry(word)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for (&k, &v) in f.iter() {
            if !b.contains(k) && k != "" {
                if v > ans.1 {
                    ans = (k.to_string(), v);
                }
            }
        }
        ans.0
    }
}
