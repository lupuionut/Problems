// 916. Word Subsets
// -----------------

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut freq = vec![0; 26];
        let mut ans = vec![];

        for i in 0..words2.len() {
            let mut freq_t = vec![0; 26];
            words2[i].chars().for_each(|c| {
                let k = c as usize - 97;
                freq_t[k] += 1;
            });
            for i in 0..26 {
                freq[i] = freq[i].max(freq_t[i]);
            }
        }

        for i in 0..words1.len() {
            let mut freq_t = vec![0; 26];
            words1[i].chars().for_each(|c| {
                let k = c as usize - 97;
                freq_t[k] += 1;
            });
            let mut ok = true;
            for i in 0..26 {
                if freq[i] > freq_t[i] {
                    ok = false;
                    break;
                }
            }
            if ok {
                ans.push(words1[i].clone());
            }
        }

        ans
    }
}
