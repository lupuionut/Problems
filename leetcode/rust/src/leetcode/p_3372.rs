// 3372. Maximize the Number of Target Nodes After Connecting Trees I
// ------------------------------------------------------------------

use std::collections::VecDeque;
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let mut ans = vec![1; n];
        let mut adj1 = vec![vec![]; n];
        let mut adj2 = vec![vec![]; m];
        let mut rank1 = vec![0; n];
        let mut best2 = vec![0; m];

        for edge in &edges1 {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj1[u].push(v);
            adj1[v].push(u);
        }

        for edge in &edges2 {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj2[u].push(v);
            adj2[v].push(u);
        }

        for i in 0..n {
            let mut q = VecDeque::new();
            q.push_back((i as i32, -1i32, 0usize));

            while q.len() > 0 {
                if let Some((node, parent, lvl)) = q.pop_front() {
                    if lvl as i32 <= k {
                        rank1[i] += 1;
                    }
                    for &next in &adj1[node as usize] {
                        if next as i32 != parent {
                            q.push_back((next as i32, node as i32, lvl + 1));
                        }
                    }
                }
            }
        }

        for i in 0..m {
            let mut curr = vec![0; m];
            let mut q = VecDeque::new();
            q.push_back((i as i32, -1i32, 0usize));

            while q.len() > 0 {
                if let Some((node, parent, lvl)) = q.pop_front() {
                    curr[lvl] += 1;
                    for &next in &adj2[node as usize] {
                        if next as i32 != parent {
                            q.push_back((next as i32, node as i32, lvl + 1));
                        }
                    }
                }
            }

            let mut acc = 0;
            for i in 1..m {
                if curr[i] == 0 {
                    break;
                }
                best2[i] = best2[i].max(curr[i] + acc);
                acc += curr[i];
            }
        }

        if k > 0 {
            for i in 0..n {
                let mut b = 0;
                let max = m.min(k as usize);
                for j in 0..max {
                    b = b.max(best2[j]);
                }
                ans[i] += rank1[i] + b;
            }
        }

        ans
    }
}
