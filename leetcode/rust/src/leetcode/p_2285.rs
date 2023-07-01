// 2285. Maximum Total Importance of Roads
// ---------------------------------------

impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut indegree = vec![0; n as usize];
        let mut adj = vec![vec![]; n as usize];
        let mut weights = vec![0; n as usize];
        let mut weight = n;
        let mut ans = 0;

        for i in 0..roads.len() {
            let cities = &roads[i];
            indegree[cities[0] as usize] += 1;
            indegree[cities[1] as usize] += 1;
            adj[cities[0] as usize].push(cities[1]);
        }
        let mut indegree: Vec<(usize, i32)> =
            indegree.iter().enumerate().map(|(i, v)| (i, *v)).collect();
        indegree.sort_by(|a, b| b.1.cmp(&a.1));
        for node in &indegree {
            weights[node.0] = weight;
            weight -= 1;
        }

        for i in 0..n as usize {
            for node in &adj[i] {
                ans += (weights[*node as usize] + weights[i]) as i64
            }
        }

        ans
    }
}
