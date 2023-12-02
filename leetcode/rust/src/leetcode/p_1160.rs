// 1160. Find Words That Can Be Formed by Characters
// -------------------------------------------------

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut freq = vec![0; 26];
        chars.chars().for_each(|c| {
            let key = c as usize - 97;
            freq[key] += 1;
        });

        let mut ans = 0;
        words.iter().for_each(|w| {
            let mut freq_clone = freq.clone();
            let mut can_form = true;
            w.chars().for_each(|c| {
                let key = c as usize - 97;
                freq_clone[key] -= 1;
                if freq_clone[key] < 0 {
                    can_form = false;
                }
            });
            if can_form == true {
                ans += w.len();
            }
        });

        ans as i32
    }
}
