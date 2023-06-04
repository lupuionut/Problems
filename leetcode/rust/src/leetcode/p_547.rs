// 547. Number of Provinces
// ------------------------

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut visited: Vec<usize> = vec![0; n];
        let mut provinces = 0;

        fn dfs(i: usize, visited: &mut Vec<usize>, adj: &Vec<Vec<usize>>) -> () {
            visited[i] = 1;
            for j in 0..adj[i].len() {
                if visited[adj[i][j]] == 0 {
                    dfs(adj[i][j], visited, adj);
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    adj[i].push(j);
                }
            }
        }

        for i in 0..n {
            if visited[i] == 0 {
                provinces += 1;
                dfs(i, &mut visited, &adj);
            }
        }

        provinces
    }
}
