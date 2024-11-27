// 3243. Shortest Distance After Road Addition Queries I
// -----------------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; n as usize];
        let mut ans = vec![];

        for i in 0..n - 1 {
            adj[i as usize].push(i + 1);
        }

        for i in 0..queries.len() {
            adj[queries[i][0] as usize].push(queries[i][1]);
            let mut queue = VecDeque::new();
            let mut visited = vec![false; n as usize];
            queue.push_back((0, 0));

            while queue.len() > 0 {
                if let Some((dist, node)) = queue.pop_front() {
                    if node == n - 1 {
                        ans.push(dist);
                        break;
                    }
                    visited[node as usize] = true;
                    for &ad in &adj[node as usize] {
                        if visited[ad as usize] == false {
                            queue.push_back((dist + 1, ad));
                        }
                    }
                }
            }
        }

        ans
    }
}
