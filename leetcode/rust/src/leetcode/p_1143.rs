// 1143. Longest Common Subsequence
// --------------------------------

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn get_longest(
            i1: usize,
            i2: usize,
            text1: &str,
            text2: &str,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i1 == text1.len() || i2 == text2.len() {
                return 0;
            }
            if cache[i1][i2] != -1 {
                return cache[i1][i2];
            }
            let mut longest = 0;
            longest = longest.max(get_longest(i1 + 1, i2, text1, text2, cache));
            longest = longest.max(get_longest(i1, i2 + 1, text1, text2, cache));
            if text1[i1..i1 + 1] == text2[i2..i2 + 1] {
                longest = longest.max(1 + get_longest(i1 + 1, i2 + 1, text1, text2, cache));
            }
            cache[i1][i2] = longest;
            cache[i1][i2]
        }

        let mut cache = vec![vec![-1; text2.len()]; text1.len()];
        get_longest(0, 0, text1.as_str(), text2.as_str(), &mut cache)
    }
}
