// 712. Minimum ASCII Delete Sum for Two Strings
// ---------------------------------------------

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut cache = vec![vec![-1; s2.len() + 1]; s1.len() + 1];

        fn get_min(s1: &[u8], s2: &[u8], i: usize, j: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
            if i == s1.len() && j == s2.len() {
                return 0;
            }
            if cache[i][j] != -1 {
                return cache[i][j];
            }
            if i == s1.len() {
                cache[i][j] = get_min(s1, s2, i, j + 1, cache) + (s2[j] as i32);
                return cache[i][j];
            }
            if j == s2.len() {
                cache[i][j] = get_min(s1, s2, i + 1, j, cache) + (s1[i] as i32);
                return cache[i][j];
            }

            let mut best = std::i32::MAX;

            if s1[i] == s2[j] {
                best = best.min(get_min(s1, s2, i + 1, j + 1, cache));
            }
            best = best.min(get_min(s1, s2, i + 1, j, cache) + (s1[i] as i32));
            best = best.min(get_min(s1, s2, i, j + 1, cache) + (s2[j] as i32));
            cache[i][j] = best;
            cache[i][j]
        }

        get_min(s1, s2, 0, 0, &mut cache)
    }
}
