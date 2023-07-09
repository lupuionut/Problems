// 2272. Substring With Largest Variance
// -------------------------------------
// kadane algo

use std::collections::HashSet;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let mut freq = vec![0; 26];
        let mut found: HashSet<u8> = HashSet::new();
        let mut bytes = s.as_bytes();

        for i in 0..bytes.len() {
            let k = bytes[i] - 97;
            freq[k as usize] += 1;
            found.insert(k);
        }

        fn kadane(small: u8, big: u8, bytes: &[u8], freq: &Vec<i32>) -> i32 {
            let mut res = 0;
            let mut count = 0;
            let mut total_small = freq[small as usize];
            let mut found_small = false;
            let mut small = 97 + small;
            let mut big = 97 + big;

            for &b in bytes {
                if b == small {
                    count -= 1;
                    total_small -= 1;
                    found_small = true;
                }
                if b == big {
                    count += 1;
                }
                if count < 0 && total_small > 0 {
                    count = 0;
                    found_small = false;
                }
                if found_small == true {
                    res = res.max(count);
                }
            }

            res
        }

        let mut ans = 0;
        for c0 in found.iter() {
            for c1 in found.iter() {
                if c0 == c1 {
                    continue;
                }
                ans = ans.max(kadane(*c0, *c1, &bytes, &freq));
            }
        }

        ans
    }
}
