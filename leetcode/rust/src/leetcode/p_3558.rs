// 3558. Number of Ways to Assign Edge Weights I
// ---------------------------------------------
use std::collections::HashMap;
impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        fn ways(i: usize, cost: i32, cache: &mut HashMap<usize, i32>) -> i32 {
            if i == 0 {
                if cost % 2 == 1 {
                    return 1;
                }
                return 0;
            }
            if let Some(&v) = cache.get(&i) {
                return v;
            }
            let mut ans = 0;
            ans += ways(i - 1, cost + 1, cache);
            ans += ways(i - 1, cost + 2, cache);
            cache.insert(i, ans % 1_000_000_007);
            ans % 1_000_000_007
        }

        let mut adj = vec![vec![]; edges.len() + 2];
        for edge in &edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }
        let mut depth = 0;
        let mut visited = vec![false; edges.len() + 2];
        let mut q = vec![];

        for i in 0..adj[1].len() {
            q.push((adj[1][i], 1));
            visited[1] = true;
            visited[adj[1][i] as usize] = true;
        }

        while let Some((node, lvl)) = q.pop() {
            depth = depth.max(lvl);
            for &new_node in &adj[node as usize] {
                if !visited[new_node as usize] {
                    visited[new_node as usize] = true;
                    q.push((new_node, lvl + 1));
                }
            }
        }

        let mut cache = HashMap::new();
        ways(depth, 0, &mut cache)
    }
}
