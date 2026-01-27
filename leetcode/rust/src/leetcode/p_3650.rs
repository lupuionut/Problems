// 3650. Minimum Cost Path with Edge Reversals
// -------------------------------------------
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![]; n as usize];

        for edge in edges {
            adj[edge[0] as usize].push((edge[1] as usize, edge[2]));
            adj[edge[1] as usize].push((edge[0] as usize, edge[2] * 2));
        }

        let mut seen = vec![false; n as usize];
        let mut heap = BinaryHeap::new();
        heap.push((0, 0usize));

        while let Some((dist, node)) = heap.pop() {
            if seen[node] == true {
                continue;
            }
            seen[node] = true;
            if node == (n - 1) as usize {
                return -dist;
            }
            for &(nn, dn) in &adj[node] {
                heap.push((-(dn - dist), nn));
            }
        }

        -1
    }
}
