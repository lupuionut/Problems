// 3442. Maximum Difference Between Even and Odd Frequency I
// ---------------------------------------------------------

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [0; 26];
        s.chars().for_each(|c| {
            let k = c as usize - 97;
            freq[k] += 1;
        });
        let mut a1 = -1i32;
        let mut a2 = -1i32;

        for i in 0..26 {
            if freq[i] != 0 {
                if freq[i] % 2 == 0 {
                    if a2 == -1 {
                        a2 = freq[i];
                    } else {
                        a2 = a2.min(freq[i]);
                    }
                } else {
                    a1 = a1.max(freq[i]);
                }
            }
        }

        a1 - a2
    }
}
