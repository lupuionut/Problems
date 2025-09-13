// 3541. Find Most Frequent Vowel and Consonant
// --------------------------------------------

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut f = [0; 26];
        let mut max_v = 0;
        let mut max_c = 0;
        s.chars().for_each(|c| {
            let k = c as usize - 97;
            f[k] += 1;
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => max_v = max_v.max(f[k]),
                _ => max_c = max_c.max(f[k]),
            }
        });
        max_v + max_c
    }
}
