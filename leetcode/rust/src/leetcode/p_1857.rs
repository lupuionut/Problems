// 1857. Largest Color Value in a Directed Graph
// ---------------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        let mut adj = vec![vec![]; n];
        let mut indegree = vec![0; n];
        let mut dist = vec![vec![0; 26]; n];
        let mut queue = VecDeque::new();

        let colors = colors
            .chars()
            .map(|c| {
                let i = c as usize - 97;
                i
            })
            .collect::<Vec<usize>>();

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            indegree[v] += 1;
        }

        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
                dist[i][colors[i]] += 1;
            }
        }

        while queue.len() > 0 {
            let curr = queue.pop_front().unwrap();

            for &v in &adj[curr] {
                for i in 0..26 {
                    if i == colors[v] {
                        dist[v][i] = dist[v][i].max(dist[curr][i] + 1);
                    } else {
                        dist[v][i] = dist[v][i].max(dist[curr][i]);
                    }
                }
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        for i in 0..n {
            if indegree[i] != 0 {
                return -1;
            }
        }

        let mut best = 0;
        for i in 0..n {
            for j in 0..26 {
                best = best.max(dist[i][j]);
            }
        }
        best
    }
}
