// 2497. Maximum Star Sum of a Graph
// ---------------------------------
use std::collections::HashMap;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut e_values: HashMap<i32, Vec<i32>> = HashMap::new();
        edges.iter().for_each(|edge| {
            let u = edge[0];
            let v = edge[1];
            let uv = vals[v as usize];
            let vu = vals[u as usize];
            if uv > 0 {
                e_values
                    .entry(u)
                    .and_modify(|c| c.push(uv))
                    .or_insert(vec![uv]);
            }
            if vu > 0 {
                e_values
                    .entry(v)
                    .and_modify(|c| c.push(vu))
                    .or_insert(vec![vu]);
            }
        });

        let mut data = HashMap::new();
        for (key, mut values) in e_values {
            values.sort_by(|a, b| b.cmp(&a));
            data.insert(key, values);
        }

        let mut best = i32::MIN;
        for i in 0..vals.len() {
            let mut sum = vals[i];
            if let Some(values) = data.get(&(i as i32)) {
                let k = values.len().min(k as usize);
                sum += &values[0..k].iter().sum::<i32>();
            }
            best = best.max(sum)
        }
        best
    }
}
