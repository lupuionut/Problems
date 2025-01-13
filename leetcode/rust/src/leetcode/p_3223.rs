// 3223. Minimum Length of String After Operations
// -----------------------------------------------

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut letters = vec![0; 26];
        let mut ans = 0;

        s.chars().for_each(|c| {
            let k = c as usize - 97;
            letters[k] += 1;
        });

        for i in 0..26 {
            if letters[i] == 0 {
                continue;
            }
            if letters[i] % 2 == 0 {
                ans += 2;
            } else {
                ans += 1;
            }
        }
        ans
    }
}
