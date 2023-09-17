// 847. Shortest Path Visiting All Nodes
// -------------------------------------

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut ans = 0;
        let max = (1 << (graph.len() as i32)) - 1;

        for i in 0..graph.len() {
            q.push_back((i as i32, 1 << i, 0));
            visited.insert((i as i32, 1 << i));
        }

        while q.len() > 0 {
            let (node, mask, val) = q.pop_front().unwrap();

            if mask == max {
                ans = val;
                break;
            }

            for &n in &graph[node as usize] {
                let new_mask = mask | (1 << n);
                if !visited.contains(&(n, new_mask)) {
                    visited.insert((n, new_mask));
                    q.push_back((n, new_mask, val + 1));
                }
            }
        }

        ans
    }
}
