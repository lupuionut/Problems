// 1976. Number of Ways to Arrive at Destination
// ---------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let MOD = 1_000_000_007;
        let mut adj = vec![vec![]; n as usize];
        let mut times = vec![i64::MAX; n as usize];
        let mut count = vec![0; n as usize];
        count[0] = 1;
        let mut heap = BinaryHeap::new();
        let mut used = vec![false; n as usize];
        heap.push((Reverse(0), 0));

        for i in 0..roads.len() {
            let road = &roads[i];
            let u = road[0];
            let v = road[1];
            let t = road[2];
            adj[u as usize].push((v, t));
            adj[v as usize].push((u, t));
        }

        while heap.len() > 0 {
            if let Some((Reverse(time), node)) = heap.pop() {
                if used[node as usize] == true {
                    continue;
                }
                used[node as usize] = true;
                for &(nn, nt) in &adj[node as usize] {
                    if times[nn as usize] > times[node as usize] + nt as i64 {
                        times[nn as usize] = times[node as usize] + nt as i64;
                        count[nn as usize] = count[node as usize] % MOD;
                        heap.push((Reverse(times[nn as usize]), nn));
                    } else if times[nn as usize] == times[node as usize] + nt as i64 {
                        count[nn as usize] += (count[node as usize] % MOD);
                        count[nn as usize] %= MOD;
                    }
                }
            }
        }

        count[(n - 1) as usize] % MOD
    }
}
