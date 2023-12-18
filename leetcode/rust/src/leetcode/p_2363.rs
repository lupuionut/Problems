// 2363. Merge Similar Items
// -------------------------

use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans = Vec::new();

        items1.iter().for_each(|arr| {
            map.entry(arr[0])
                .and_modify(|count| *count += arr[1])
                .or_insert(arr[1]);
        });
        items2.iter().for_each(|arr| {
            map.entry(arr[0])
                .and_modify(|count| *count += arr[1])
                .or_insert(arr[1]);
        });

        for (k, v) in map.iter() {
            ans.push(vec![*k, *v]);
        }

        ans.sort_by(|a, b| a[0].cmp(&b[0]));
        ans
    }
}
