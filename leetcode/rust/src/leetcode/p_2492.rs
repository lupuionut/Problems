// 2492. Minimum Score of a Path Between Two Cities
// ------------------------------------------------
impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = vec![vec![]; (n+1) as usize];
        let mut costs = vec![vec![]; (n+1) as usize];
        let mut visited = vec![false; (n+1) as usize];
        let mut q = vec![];

        for i in 0..roads.len() {
            costs[roads[i][0] as usize].push(roads[i][2]);
            costs[roads[i][1] as usize].push(roads[i][2]);
            adj[roads[i][0] as usize].push(roads[i][1]);
            adj[roads[i][1] as usize].push(roads[i][0]);
        }

        q.push(1);
        visited[1] = true;
        let mut ans = i32::MAX;

        while let Some(node) = q.pop() {
            for i in 0..costs[node as usize].len() {
                ans = ans.min(costs[node as usize][i]);
            }
            for i in 0..adj[node as usize].len() {
                let next = adj[node as usize][i];
                if visited[next as usize] == false {
                    visited[next as usize] = true;
                    q.push(next);
                }
            }
        }

        ans
    }
}
