// 2900. Longest Unequal Adjacent Groups Subsequence I
// ---------------------------------------------------

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut start_0 = vec![];
        let mut start_1 = vec![];
        let mut f = 0;
        let mut s = 1;

        for i in 0..groups.len() {
            if groups[i] == 0 {
                if f == 1 {
                    start_0.push(&words[i]);
                    f = 0;
                }
                if s == 1 {
                    start_1.push(&words[i]);
                    s = 0;
                }
            } else {
                if f == 0 {
                    start_0.push(&words[i]);
                    f = 1;
                }
                if s == 0 {
                    start_1.push(&words[i]);
                    s = 1;
                }
            }
        }

        if start_0.len() > start_1.len() {
            start_0.into_iter().map(|s| s.clone()).collect()
        } else {
            start_1.into_iter().map(|s| s.clone()).collect()
        }
    }
}
