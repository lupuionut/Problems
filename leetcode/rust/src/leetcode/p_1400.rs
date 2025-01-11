// 1400. Construct K Palindrome Strings
// ------------------------------------

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut chars = vec![0; 26];

        if s.len() < k as usize {
            return false;
        }

        s.chars().for_each(|c| {
            let k = c as usize - 97;
            chars[k] += 1;
        });

        let mut odd = 0;
        for i in 0..26 {
            if chars[i] % 2 != 0 {
                odd += 1;
            }
        }

        if odd > k {
            return false;
        }

        true
    }
}
