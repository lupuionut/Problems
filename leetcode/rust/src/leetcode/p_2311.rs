// 2311. Longest Binary Subsequence Less Than or Equal to K
// --------------------------------------------------------

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut r = s.len() as i32 - 1;
        let n = r;
        let mut curr = 0;

        while r >= 0 {
            if s[r as usize] == '1' {
                curr += 1 << (n - r);
            }
            if curr <= k as i64 {
                ans += 1;
                r -= 1;
            } else {
                break;
            }
        }
        if r > 0 {
            for i in 0..r as usize {
                if s[i] == '0' {
                    ans += 1;
                }
            }
        }

        ans
    }
}
