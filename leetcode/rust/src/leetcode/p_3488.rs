// 3488. Closest Equal Element Queries
// -----------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut existing: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut ans = vec![-1; queries.len()];
        nums.iter().enumerate().for_each(|(k, &v)| {
            existing
                .entry(v)
                .and_modify(|mut vals| {
                    vals.push(k as i32);
                })
                .or_insert(vec![k as i32]);
        });
        fn matchp(keys: &Vec<i32>, q: i32) -> usize {
            let mut l = 0;
            let mut r = keys.len();

            while l < r {
                let mid = (l + r) / 2;
                if keys[mid] == q {
                    return mid;
                }
                if keys[mid] < q {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            return l;
        }
        for j in 0..queries.len() {
            let q = queries[j];
            let val = nums[q as usize];
            if let Some(keys) = existing.get(&val) {
                let n = nums.len();
                let m = keys.len();
                if m == 1 {
                    continue;
                }
                let i = matchp(keys, q);
                let k = keys[i];
                let mut best = i32::MAX;
                let left = (i - 1 + m) % m;
                if left < i {
                    best = best.min((k - keys[left]).abs());
                    best = best.min((keys[left] + n as i32 - k).abs());
                } else {
                    best = best.min((keys[left] - k).abs());
                    best = best.min((k + n as i32 - keys[left]).abs());
                }
                let right = (i + 1 + m) % m;
                if right > i {
                    best = best.min((keys[right] - k).abs());
                    best = best.min((n as i32 - keys[right] + k).abs());
                } else {
                    best = best.min((k - keys[right]).abs());
                    best = best.min((keys[right] + n as i32 - k).abs());
                }
                ans[j] = best;
            }
        }

        ans
    }
}
