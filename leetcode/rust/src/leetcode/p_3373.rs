// 3373. Maximize the Number of Target Nodes After Connecting Trees II
// -------------------------------------------------------------------

use std::collections::VecDeque;
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; edges1.len() + 1];
        fn colorize(edges: &Vec<Vec<i32>>) -> Vec<i32> {
            let n = edges.len() + 1;
            let mut color = vec![-1; n];
            let mut adj = vec![vec![]; n];
            for edge in edges {
                let u = edge[0];
                let v = edge[1];
                adj[u as usize].push(v);
                adj[v as usize].push(u);
            }
            let mut q = VecDeque::new();
            q.push_back(0);
            color[0] = 0;

            while q.len() > 0 {
                if let Some(curr) = q.pop_front() {
                    for &node in &adj[curr] {
                        if color[node as usize] == -1 {
                            color[node as usize] = (color[curr as usize] + 1) % 2;
                            q.push_back(node as usize);
                        }
                    }
                }
            }

            color
        }

        let tree1 = colorize(&edges1);
        let tree2 = colorize(&edges2);
        // sum for each color, white 0, black 1
        let mut color1 = [0, 0];
        let mut color2 = [0, 0];
        for i in 0..tree1.len() {
            color1[tree1[i] as usize] += 1;
        }
        for i in 0..tree2.len() {
            color2[tree2[i] as usize] += 1;
        }

        let s = color2[0].max(color2[1]);
        for i in 0..edges1.len() + 1 {
            let c = color1[tree1[i] as usize];
            ans[i] = c + s;
        }

        ans
    }
}
