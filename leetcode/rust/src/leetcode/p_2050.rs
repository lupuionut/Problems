// 2050. Parallel Courses III
// --------------------------

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut indegrees = vec![0; n as usize];
        let mut start = vec![0; n as usize];
        let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = VecDeque::new();

        relations.iter().for_each(|rel| {
            let u = rel[0] - 1;
            let v = rel[1] - 1;

            indegrees[v as usize] += 1;
            edges
                .entry(u)
                .and_modify(|val| val.push(v))
                .or_insert(vec![v]);
        });

        for i in 0..n {
            if indegrees[i as usize] == 0 {
                queue.push_back(i);
            }
        }

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if let Some(es_node) = edges.get(&node) {
                es_node.iter().for_each(|&e| {
                    indegrees[e as usize] -= 1;
                    start[e as usize] =
                        start[e as usize].max(start[node as usize] + time[node as usize]);
                    if indegrees[e as usize] == 0 {
                        queue.push_back(e);
                    }
                });
            }
        }

        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(time[i as usize] + start[i as usize]);
        }

        ans
    }
}
