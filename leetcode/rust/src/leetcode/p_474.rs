// 474. Ones and Zeroes
// --------------------

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut ans = 0;
        let mut cache = vec![vec![vec![-1; n as usize + 1]; m as usize + 1]; strs.len()];
        let mut strs = strs;
        strs.sort_by(|a, b| a.len().cmp(&b.len()));

        fn dfs(
            i: usize,
            strs: &Vec<String>,
            zeroes: i32,
            ones: i32,
            cache: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if zeroes == 0 && ones == 0 {
                return 0;
            }

            if i == strs.len() {
                return 0;
            }

            if cache[i][zeroes as usize][ones as usize] != -1 {
                return cache[i][zeroes as usize][ones as usize];
            }

            let mut ans = 0;
            let word = &strs[i];
            let mut zero = 0;
            let mut one = 0;

            // early terminate if already the word length is
            // larger than the sum of available zeroes and ones
            if (zeroes + ones) < word.len() as i32 {
                return 0;
            }

            word.chars().for_each(|c| {
                if c == '0' {
                    zero += 1;
                } else {
                    one += 1;
                }
            });

            if zero <= zeroes && one <= ones {
                ans = 1 + dfs(i + 1, strs, zeroes - zero, ones - one, cache);
            }

            ans = ans.max(dfs(i + 1, strs, zeroes, ones, cache));
            cache[i][zeroes as usize][ones as usize] = ans;
            ans
        }

        dfs(0, &strs, m, n, &mut cache)
    }
}
