// 2192. All Ancestors of a Node in a Directed Acyclic Graph
// ---------------------------------------------------------

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut indegree = vec![0; n as usize];
        let mut adj = vec![vec![]; n as usize];
        let mut temp = vec![HashSet::new(); n as usize];

        for i in 0..edges.len() {
            let edge = &edges[i];
            indegree[edge[1] as usize] += 1;
            adj[edge[0] as usize].push(edge[1]);
        }

        let mut queue = VecDeque::new();
        for i in 0..n as usize {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            for i in 0..adj[node].len() {
                let child = adj[node][i];
                temp[child as usize].insert(node as i32);
                for n in temp[node as usize].clone() {
                    temp[child as usize].insert(n);
                }
                indegree[child as usize] -= 1;

                if indegree[child as usize] == 0 {
                    queue.push_back(child as usize);
                }
            }
        }

        temp.into_iter()
            .map(|t| {
                let mut t = t.into_iter().collect::<Vec<i32>>();
                t.sort();
                t
            })
            .collect::<Vec<_>>()
    }
}
