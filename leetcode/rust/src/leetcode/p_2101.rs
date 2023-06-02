// 2101. Detonate the Maximum Bombs
// --------------------------------

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut adj: Vec<Vec<usize>> = vec![vec![]; bombs.len()];
        let mut maximum = 0;

        fn dfs(i: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<i32>) -> i32 {
            visited[i] = 1;
            let mut total = 1;
            for next in &adj[i] {
                if visited[*next] == 1 {
                    continue;
                }
                total += dfs(*next, adj, visited);
            }
            return total;
        }

        for i in 0..bombs.len() {
            for j in 0..bombs.len() {
                if i == j {
                    continue;
                }
                let r: u64 = ((bombs[i][0] - bombs[j][0]) as u64).pow(2)
                    + ((bombs[i][1] - bombs[j][1]) as u64).pow(2);
                if r <= bombs[i][2].pow(2) as u64 {
                    adj[i].push(j);
                }
            }
        }

        for i in 0..bombs.len() {
            let mut visited = vec![0; bombs.len()];
            let current_max = dfs(i, &adj, &mut visited);

            if current_max > maximum {
                maximum = current_max;
            }
        }

        maximum
    }
}
