// 647. Palindromic Substrings
// ---------------------------

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = 0;
        let n = s.len() as i32;

        for i in 0..n {
            let mut left = i;
            let mut right = i;
            while left >= 0 && right < n {
                if s[left as usize] == s[right as usize] {
                    ans += 1;
                } else {
                    break;
                }
                left -= 1;
                right += 1;
            }

            left = i;
            right = i + 1;
            while left >= 0 && right < n {
                if s[left as usize] == s[right as usize] {
                    ans += 1;
                } else {
                    break;
                }
                left -= 1;
                right += 1;
            }
        }

        ans
    }
}
