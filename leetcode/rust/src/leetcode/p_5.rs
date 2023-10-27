// 5. Longest Palindromic Substring
// --------------------------------

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut ans: Vec<char> = vec![];
        let n = chars.len();

        for middle in 0..n {
            // odd
            let mut left = middle as i32;
            let mut right = middle as i32;

            while left >= 0 && right < (n as i32) && chars[left as usize] == chars[right as usize] {
                left -= 1;
                right += 1;
            }

            if (right - left - 1) as usize > ans.len() {
                ans = chars[left as usize + 1..right as usize].to_vec();
            }

            // even
            left = middle as i32;
            right = middle as i32 + 1;

            if right >= chars.len() as i32 || chars[left as usize] != chars[right as usize] {
                continue;
            }

            while left >= 0 && right < (n as i32) && chars[left as usize] == chars[right as usize] {
                left -= 1;
                right += 1;
            }

            if (right - left - 1) as usize > ans.len() {
                ans = chars[left as usize + 1..right as usize].to_vec();
            }
        }

        ans.iter().collect::<String>()
    }
}
