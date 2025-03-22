// 2685. Count the Number of Complete Components
// ---------------------------------------------

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut indegrees = vec![0; n as usize];
        let mut adj = vec![vec![]; n as usize];
        let mut connected = vec![];

        edges.iter().for_each(|edge| {
            let u = edge[0];
            let v = edge[1];
            indegrees[u as usize] += 1;
            indegrees[v as usize] += 1;
            adj[u as usize].push(v);
            adj[v as usize].push(u);
        });

        let mut visited = vec![false; n as usize];

        fn traverse(
            node: i32,
            adj: &Vec<Vec<i32>>,
            visited: &mut Vec<bool>,
            connected: &mut Vec<Vec<i32>>,
        ) {
            let key = connected.len();
            connected.push(vec![]);
            connected[key].push(node);
            let mut stack = vec![];
            stack.push(node);

            while stack.len() > 0 {
                let curr = stack.pop().unwrap();
                for &n in &adj[curr as usize] {
                    if visited[n as usize] == false {
                        stack.push(n);
                        visited[n as usize] = true;
                        connected[key].push(n);
                    }
                }
            }
        }

        for i in 0..n {
            if visited[i as usize] == false {
                visited[i as usize] = true;
                traverse(i, &adj, &mut visited, &mut connected);
            }
        }

        for i in 0..connected.len() {
            let n = connected[i].len();
            let degree = n - 1;
            let mut correct = true;
            for j in 0..n {
                let key = connected[i][j] as usize;
                if indegrees[key] != degree {
                    correct = false;
                    break;
                }
            }
            if correct {
                ans += 1;
            }
        }

        ans
    }
}
