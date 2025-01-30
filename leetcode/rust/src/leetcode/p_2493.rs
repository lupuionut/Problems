// 2493. Divide Nodes Into the Maximum Number of Groups
// ----------------------------------------------------

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![]; (n + 1) as usize];
        let mut visited = vec![false; (n + 1) as usize];
        let mut components = vec![];

        edges.iter().for_each(|e| {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        });

        fn add_to_component(
            n: i32,
            adj: &Vec<Vec<i32>>,
            components: &mut Vec<Vec<i32>>,
            visited: &mut Vec<bool>,
        ) {
            let mut conn = vec![];
            let mut q = vec![n];
            while q.len() > 0 {
                let x = q.pop().unwrap();
                conn.push(x);
                visited[x as usize] = true;
                for &a in &adj[x as usize] {
                    if !visited[a as usize] {
                        visited[a as usize] = true;
                        q.push(a);
                    }
                }
            }

            components.push(conn);
        }

        fn max_component(component: &Vec<i32>, adj: &Vec<Vec<i32>>) -> i32 {
            let mut best = -1;
            for i in 0..component.len() {
                let r = component[i];
                let mut dist = HashMap::new();
                dist.insert(r, 1);
                let mut q = VecDeque::new();
                q.push_back((r, 1));

                while q.len() > 0 {
                    if let Some((r, d)) = q.pop_front() {
                        best = best.max(d);
                        for &a in &adj[r as usize] {
                            if let Some(&dd) = dist.get(&a) {
                                if dd == d {
                                    return -1;
                                }
                                continue;
                            }
                            q.push_back((a, d + 1));
                            dist.insert(a, d + 1);
                        }
                    }
                }
            }

            best
        }

        for i in 1..=n {
            if !visited[i as usize] {
                add_to_component(i, &adj, &mut components, &mut visited)
            }
        }

        let mut ans = 0;
        for i in 0..components.len() {
            let t = max_component(&components[i], &adj);
            if t == -1 {
                return -1;
            }
            ans += t;
        }

        ans
    }
}
