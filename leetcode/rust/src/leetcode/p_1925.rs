// 1925. Count Square Sum Triples
// ------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        let mut squares = HashSet::new();
        let mut i = 1;
        for i in 1..=n {
            squares.insert(i * i);
        }
        for a in 1..=n {
            for b in 1..=n {
                let c = a * a + b * b;
                if squares.contains(&c) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
