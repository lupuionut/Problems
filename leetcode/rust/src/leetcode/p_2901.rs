// 2901. Longest Unequal Adjacent Groups Subsequence II
// ----------------------------------------------------

impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        fn h_distance(s1: &String, s2: &String) -> i32 {
            let mut d = 0;
            let mut iter = s1.chars().zip(s2.chars());
            if s1.len() != s2.len() {
                return -1;
            }
            while let Some((c1, c2)) = iter.next() {
                if c1 != c2 {
                    d += 1;
                }
            }
            d
        }

        let mut dp = vec![1; words.len()];
        let mut path = vec![None; words.len()];
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if groups[i] != groups[j] && h_distance(&words[i], &words[j]) == 1 {
                    if dp[j] < dp[i] + 1 {
                        dp[j] = dp[i] + 1;
                        path[j] = Some(i);
                    }
                }
            }
        }

        let mut start = 0;
        for i in 0..dp.len() {
            if dp[i] > dp[start] {
                start = i;
            }
        }

        let mut ans = vec![];
        loop {
            ans.push(words[start].clone());
            if let Some(s) = path[start] {
                start = s;
            } else {
                break;
            }
        }

        ans.into_iter().rev().collect()
    }
}
