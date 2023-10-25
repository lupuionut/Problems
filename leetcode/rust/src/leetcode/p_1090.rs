// 1090. Largest Values From Labels
// --------------------------------
// Greedely take the largest values while
// the label counter is less than limit

use std::collections::HashMap;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut iter = values.iter().zip(labels.iter());
        let mut num_wanted = num_wanted;
        let mut zipped = vec![];
        let mut ans = 0;
        let mut used = HashMap::new();
        let mut i = 0;

        while let Some((v, l)) = iter.next() {
            zipped.push((v, l));
        }
        zipped.sort_by(|a, b| b.0.cmp(&a.0));

        while i < zipped.len() && num_wanted > 0 {
            let (val, lbl) = zipped[i];
            if let Some(&use_count) = used.get(&lbl) {
                if use_count < use_limit {
                    ans += val;
                    num_wanted -= 1;
                    used.insert(lbl, use_count + 1);
                }
            } else {
                used.insert(lbl, 1);
                ans += val;
                num_wanted -= 1;
            }
            i += 1;
        }

        ans
    }
}
