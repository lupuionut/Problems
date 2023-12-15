// 1436. Destination City
// ----------------------

use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut outgoing = HashSet::new();
        let mut ans = String::new();

        paths.iter().for_each(|pair| {
            outgoing.insert(&pair[0]);
        });

        paths.iter().for_each(|pair| {
            if !outgoing.contains(&pair[1]) {
                ans = pair[1].clone();
            }
        });

        ans
    }
}
