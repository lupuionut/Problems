// 1443. Minimum Time to Collect All Apples in a Tree
// --------------------------------------------------

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut adj = vec![vec![]; n as usize];

        edges.iter().for_each(|e| {
            adj[e[0] as usize].push(e[1]);
            adj[e[1] as usize].push(e[0]);
        });

        fn dfs(node: i32, parent: i32, adj: &Vec<Vec<i32>>, has: &Vec<bool>) -> i32 {
            let mut time = 0;

            for &n in &adj[node as usize] {
                if n == parent {
                    continue;
                }
                let c_time = dfs(n, node, adj, has);
                if c_time > 0 || has[n as usize] == true {
                    time += 2 + c_time;
                }
            }
            time
        }

        dfs(0, -1, &adj, &has_apple)
    }
}
