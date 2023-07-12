// 802. Find Eventual Safe States
// ------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut degree = vec![0; graph.len()];
        let mut connects = vec![vec![]; graph.len()];
        let mut ans = vec![];

        for i in 0..graph.len() {
            let adj = &graph[i];
            degree[i] = adj.len();
            for n in adj {
                connects[*n as usize].push(i);
            }
            if degree[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        while queue.len() > 0 {
            let i = queue.pop_front().unwrap();
            ans.push(i);

            for connection in &connects[i as usize] {
                degree[*connection] -= 1;
                if degree[*connection] == 0 {
                    queue.push_back(*connection as i32);
                }
            }
        }

        ans.sort();
        ans
    }
}
