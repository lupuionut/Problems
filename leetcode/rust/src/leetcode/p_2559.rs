// 2559. Count Vowel Strings in Ranges
// -----------------------------------

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ps: Vec<i32> = vec![0; words.len()];
        let mut ans: Vec<i32> = vec![];

        for i in 0..words.len() {
            let mut iter = words[i].chars();
            let first = iter.nth(0).unwrap();
            let last = iter.last().or(Some(first)).unwrap();
            let is_vowel = match (first, last) {
                ('a', 'a') | ('a', 'e') | ('a', 'i') | ('a', 'o') | ('a', 'u') => true,
                ('e', 'a') | ('e', 'e') | ('e', 'i') | ('e', 'o') | ('e', 'u') => true,
                ('i', 'a') | ('i', 'e') | ('i', 'i') | ('i', 'o') | ('i', 'u') => true,
                ('o', 'a') | ('o', 'e') | ('o', 'i') | ('o', 'o') | ('o', 'u') => true,
                ('u', 'a') | ('u', 'e') | ('u', 'i') | ('u', 'o') | ('u', 'u') => true,
                _ => false,
            };

            if i > 0 {
                if is_vowel {
                    ps[i] = ps[i - 1] + 1;
                } else {
                    ps[i] = ps[i - 1];
                }
            } else {
                if is_vowel {
                    ps[i] = 1;
                }
            }
        }

        for query in queries {
            let start = if query[0] == 0 {
                0
            } else {
                ps[query[0] as usize - 1]
            };
            let end = ps[query[1] as usize];
            ans.push(end - start);
        }

        ans
    }
}
