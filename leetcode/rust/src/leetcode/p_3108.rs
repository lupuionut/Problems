// 3108. Minimum Cost Walk in Weighted Graph
// -----------------------------------------

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; n as usize];
        let mut ans = vec![-1; query.len()];
        let mut visited = vec![false; n as usize];
        let mut root = vec![-1; n as usize];
        let mut cost = HashMap::new();

        edges.iter().for_each(|e| {
            adj[e[0] as usize].push((e[1], e[2]));
            adj[e[1] as usize].push((e[0], e[2]));
        });

        fn visit(
            i: usize,
            adj: &Vec<Vec<(i32, i32)>>,
            visited: &mut Vec<bool>,
            root: &mut Vec<i32>,
            cost: &mut HashMap<i32, i32>,
        ) {
            let mut q = VecDeque::new();
            let mut total = i32::MAX;
            q.push_back(i);
            visited[i] = true;

            while q.len() > 0 {
                if let Some(curr) = q.pop_front() {
                    root[curr as usize] = i as i32;
                    for &(n, w) in &adj[curr] {
                        total &= w;
                        if visited[n as usize] == false {
                            q.push_back(n as usize);
                            visited[n as usize] = true;
                        }
                    }
                }
            }

            cost.insert(i as i32, total);
        }

        for i in 0..n as usize {
            if !visited[i] {
                visit(i, &adj, &mut visited, &mut root, &mut cost);
            }
        }

        for i in 0..query.len() {
            let u = query[i][0];
            let v = query[i][1];
            if root[u as usize] == root[v as usize] {
                let r = root[u as usize];
                let c = *cost.get(&r).unwrap();
                ans[i] = c;
            }
        }

        ans
    }
}
