// 1615. Maximal Network Rank
// --------------------------

use std::collections::HashSet;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut indegree = vec![0; n as usize];
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n as usize];
        let mut connected: HashSet<(i32, i32)> = HashSet::new();
        let mut max = 0;

        for road in roads {
            let u = road[0];
            let v = road[1];
            indegree[u as usize] += 1;
            indegree[v as usize] += 1;
            adj[u as usize].push(v);
            adj[v as usize].push(u);
            connected.insert((u, v));
            connected.insert((v, u));
        }

        for i in 0..n {
            for j in i + 1..n {
                let mut total = indegree[i as usize] + indegree[j as usize];
                if connected.contains(&(i, j)) {
                    total -= 1;
                }
                if total > max {
                    max = total;
                }
            }
        }

        max
    }
}
