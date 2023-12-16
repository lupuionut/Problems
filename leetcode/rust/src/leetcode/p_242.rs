// 242. Valid Anagram
// ------------------

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut letters = vec![0; 26];
        s.as_bytes().iter().for_each(|&letter| {
            let key = letter as usize - 97;
            letters[key] += 1;
        });

        t.as_bytes().iter().for_each(|&letter| {
            let key = letter as usize - 97;
            letters[key] -= 1;
        });

        let mut ans = true;
        for i in 0..26 {
            if letters[i] != 0 {
                ans = false;
                break;
            }
        }

        ans
    }
}
