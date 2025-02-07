// 3160. Find the Number of Distinct Colors Among the Balls
// --------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut colors = HashMap::new();
        let mut ans = vec![0; queries.len()];

        for i in 0..queries.len() {
            let b = queries[i][0] as usize;
            let c = queries[i][1];

            if let Some(&col) = colors.get(&b) {
                // if ball is already colored with this colour
                if c == col {
                    ans[i] = map.len() as i32;
                    continue;
                } else {
                    if let Some(&q) = map.get(&col) {
                        if q == 1 {
                            map.remove(&col);
                        } else {
                            map.entry(col).and_modify(|c| *c -= 1);
                        }
                    }
                }
            }

            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
            colors.entry(b).and_modify(|col| *col = c).or_insert(c);
            ans[i] = map.len() as i32;
        }

        ans
    }
}
