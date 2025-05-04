// 1128. Number of Equivalent Domino Pairs
// ---------------------------------------

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut connected = [[0; 10]; 10];

        for i in 0..dominoes.len() {
            let u = dominoes[i][0] as usize;
            let v = dominoes[i][1] as usize;
            if u <= v {
                connected[u][v] += 1;
                ans += connected[u][v] - 1;
            } else {
                connected[v][u] += 1;
                ans += connected[v][u] - 1;
            }
        }
        ans
    }
}
