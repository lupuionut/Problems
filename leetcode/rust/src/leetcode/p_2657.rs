// 2657. Find the Prefix Common Array of Two Arrays

use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut seen: HashSet<i32> = HashSet::new();
        let mut acc: Vec<i32> = Vec::new();
        acc.push(0);

        for i in 0..a.len() {
            let mut t = acc[i];
            if a[i] == b[i] {
                t += 1;
            } else {
                if seen.contains(&a[i]) {
                    t += 1;
                } else {
                    seen.insert(a[i]);
                }

                if seen.contains(&b[i]) {
                    t += 1;
                } else {
                    seen.insert(b[i]);
                }
            }
            acc.push(t);
        }

        acc.drain(1..).collect()
    }
}
