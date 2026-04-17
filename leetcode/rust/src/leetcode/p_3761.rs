// 3761. Minimum Absolute Distance Between Mirror Pairs
// ----------------------------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        fn reverse(mut i: i32) -> i32 {
            let mut digits = vec![];
            let mut ans = 0;
            while i > 0 {
                if digits.len() > 0 || i % 10 != 0 {
                    digits.push(i % 10);
                }
                i /= 10;
            }
            for i in 0..digits.len() {
                ans *= 10;
                ans += digits[i];
            }
            ans
        }
        let reversed: Vec<i32> = nums.iter().map(|&x| reverse(x)).collect();
        let mut last_seen: HashMap<i32, usize> = HashMap::new();
        let mut best = None;

        for j in 0..nums.len() {
            if let Some(i) = last_seen.get(&nums[j]) {
                let d = (j - i) as i32;
                if let Some(b) = best {
                    if b > d {
                        best = Some(d);
                    }
                } else {
                    best = Some(d);
                }
            }
            last_seen
                .entry(reversed[j])
                .and_modify(|v| *v = j)
                .or_insert(j);
        }

        if let Some(d) = best {
            return d;
        }
        -1
    }
}
