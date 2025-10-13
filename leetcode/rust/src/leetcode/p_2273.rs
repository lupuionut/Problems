// 2273. Find Resultant Array After Removing Anagrams
// --------------------------------------------------

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        fn normalize(w: &String) -> [usize; 26] {
            let mut ans = [0; 26];
            w.chars().for_each(|c| {
                let k = c as usize - 97;
                ans[k] += 1;
            });
            ans
        }

        let mut stack: Vec<(&String, [usize; 26])> = vec![];
        words.iter().for_each(|w| {
            let f = normalize(w);
            if stack.len() == 0 {
                stack.push((w, f));
            } else {
                let n = stack.len() - 1;
                if stack[n].1 != f {
                    stack.push((w, f));
                }
            }
        });
        stack.into_iter().map(|(w, _)| w.clone()).collect()
    }
}
