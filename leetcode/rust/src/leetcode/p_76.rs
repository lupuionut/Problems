// 76. Minimum Window Substring
// ----------------------------

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_freq = vec![0; 52];
        let mut range_freq = vec![0; 52];
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut left = 0;
        let mut best_len = s.len();
        let mut best_index = None;

        for i in 0..t.len() {
            let key = Solution::key_from_char(t[i]);
            t_freq[key] += 1;
        }

        for right in 0..s.len() {
            let key = Solution::key_from_char(s[right]);
            range_freq[key] += 1;

            // check if all chars in curr range are >= than target freq
            let mut ok = true;
            for i in 0..t_freq.len() {
                if range_freq[i] < t_freq[i] {
                    ok = false;
                    break;
                }
            }

            if ok {
                let mut key = Solution::key_from_char(s[left]);
                while range_freq[key] > t_freq[key] {
                    range_freq[key] -= 1;
                    left += 1;
                    key = Solution::key_from_char(s[left]);
                }
                if right - left + 1 <= best_len {
                    best_len = right - left + 1;
                    best_index = Some(left);
                }
            }
        }
        //println!("{:?}", (best_index, best_len));
        if let Some(i) = best_index {
            return s[i..i + best_len].into_iter().collect::<String>();
        }

        "".to_string()
    }

    fn key_from_char(c: char) -> usize {
        let mut base = 0;
        if c.is_lowercase() {
            base = 26 + (c as usize) - 97;
        } else {
            base = c as usize - 65;
        }
        base
    }
}
