// 1347. Minimum Number of Steps to Make Two Strings Anagram
// ---------------------------------------------------------

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_chars = vec![0; 26];
        let mut t_chars = vec![0; 26];

        s.chars().for_each(|c| {
            let key = (c as usize) - 97;
            s_chars[key] += 1;
        });

        t.chars().for_each(|c| {
            let key = (c as usize) - 97;
            t_chars[key] += 1;
        });

        let mut common = 0;
        for i in 0..26 {
            if t_chars[i] > s_chars[i] {
                common += s_chars[i];
            } else {
                common += t_chars[i];
            }
        }

        s.len() as i32 - common
    }
}
