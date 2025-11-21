// 1930. Unique Length-3 Palindromic Subsequences
// ----------------------------------------------
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut ans = 0;
        let mut letters = vec![vec![]; 26];
        s.chars().enumerate().for_each(|(p, c)| {
            let k = c as usize - 97;
            letters[k].push(p);
        });

        for i in 0..26 {
            let n = letters[i].len();
            if n >= 2 {
                if n > 2 {
                    ans += 1;
                }
                for j in 0..26 {
                    if i == j {
                        continue;
                    }
                    if contains(&letters[j], letters[i][0], letters[i][n - 1]) {
                        ans += 1;
                    }
                }
            }
        }

        fn contains(vals: &Vec<usize>, start: usize, end: usize) -> bool {
            if vals.len() == 0 {
                return false;
            }
            let mut l = 0;
            let mut r = vals.len();
            while l < r {
                let mid = (l + r) / 2;
                if vals[mid] > start && vals[mid] < end {
                    return true;
                }
                if vals[mid] < start {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            false
        }
        ans
    }
}
