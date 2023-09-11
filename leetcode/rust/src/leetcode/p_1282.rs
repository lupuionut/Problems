// 1282. Group the People Given the Group Size They Belong To
// ----------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut temp: HashMap<i32, Vec<i32>> = HashMap::new();

        group_sizes.iter().enumerate().for_each(|(k, &v)| {
            temp.entry(v)
                .and_modify(|e| {
                    e.push(k as i32);
                })
                .or_insert(vec![k as i32]);
            if let Some(e) = temp.get(&v) {
                if e.len() == v as usize {
                    ans.push(e.to_vec());
                    temp.remove(&v);
                }
            }
        });

        ans
    }
}
