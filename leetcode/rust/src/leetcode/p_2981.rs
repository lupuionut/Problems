// 2981. Find Longest Special Substring That Occurs Thrice I
// ---------------------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut substr = HashMap::new();

        for i in 0..s.len() {
            for j in (i + 1)..=s.len() {
                substr.entry(&s[i..j]).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let mut ans = -1;
        substr
            .into_iter()
            .filter(|(k, v)| {
                if *v < 3 {
                    return false;
                }
                let mut it = k.chars();
                let first = it.next().unwrap();
                it.all(|c| c == first)
            })
            .for_each(|(k, v)| ans = ans.max(k.len() as i32));

        ans
    }
}
