// 3583. Count Special Triplets
// ----------------------------
use std::collections::HashMap;
impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<(i32, i32), i64> = HashMap::new();
        let mut ans = 0;
        for &num in &nums {
            if num % 2 == 0 {
                let j = num / 2;
                if let Some(v) = counter.get(&(2, j)) {
                    ans += v;
                    ans %= 1_000_000_007;
                }
            }

            if let Some(&mut v) = counter.get_mut(&(1, num * 2)) {
                counter
                    .entry((2, num))
                    .and_modify(|c| {
                        *c += v;
                        *c %= 1_000_000_007
                    })
                    .or_insert(v);
            }
            counter
                .entry((1, num))
                .and_modify(|v| {
                    *v += 1;
                    *v %= 1_000_000_007
                })
                .or_insert(1);
        }
        ans as i32
    }
}
