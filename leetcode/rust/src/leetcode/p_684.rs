// 684. Redundant Connection
// -------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj = vec![vec![]; edges.len() + 1];
        let mut cycle = HashSet::new();

        edges.iter().enumerate().for_each(|(k, v)| {
            adj[v[0] as usize].push(v[1]);
            adj[v[1] as usize].push(v[0]);
        });

        fn visit(
            node: i32,
            parent: i32,
            adj: &Vec<Vec<i32>>,
            seen: &mut Vec<bool>,
            path: &mut Vec<i32>,
            cycle: &mut HashSet<(i32, i32)>,
        ) {
            if seen[node as usize] == true {
                let mut p = parent;
                cycle.insert((node, p));
                cycle.insert((p, node));
                while p != node {
                    let t = p;
                    p = path[p as usize];
                    cycle.insert((t, p));
                    cycle.insert((p, t));
                }
                return;
            }
            seen[node as usize] = true;
            path[node as usize] = parent;
            for &n in &adj[node as usize] {
                if n != parent {
                    if cycle.len() == 0 {
                        visit(n, node, adj, seen, path, cycle);
                    }
                }
            }
        }

        for i in 1..(edges.len() + 1) {
            let u = edges[i][0];
            let v = edges[i][1];
            let mut seen = vec![false; edges.len() + 1];
            let mut path = vec![-1; edges.len() + 1];
            seen[u as usize] = true;
            path[u as usize] = -1;

            if cycle.len() == 0 {
                visit(v, u, &adj, &mut seen, &mut path, &mut cycle);
            } else {
                break;
            }
        }

        for i in (0..edges.len()).rev() {
            let u = edges[i][0];
            let v = edges[i][1];
            if cycle.contains(&(u, v)) || cycle.contains(&(v, u)) {
                return edges[i].clone();
            }
        }
        vec![]
    }
}
