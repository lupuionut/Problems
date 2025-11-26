// 2435. Paths in Matrix Whose Sum Is Divisible by K
// -------------------------------------------------
impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cache = vec![vec![vec![-1; (k + 1) as usize]; grid[0].len()]; grid.len()];
        fn visit(
            i: usize,
            j: usize,
            grid: &Vec<Vec<i32>>,
            md: i32,
            k: i32,
            cache: &mut Vec<Vec<Vec<i64>>>,
        ) -> i64 {
            if cache[i][j][md as usize] != -1 {
                return cache[i][j][md as usize];
            }
            let c = grid[0].len() - 1;
            let r = grid.len() - 1;
            if i == r && j == c {
                if md == 0 {
                    return 1;
                }
                return 0;
            }
            let mut ans = 0;
            if i + 1 <= r {
                let new = (md + grid[i + 1][j]) % k;
                ans += visit(i + 1, j, grid, new, k, cache);
                ans %= 1_000_000_007;
            }
            if j + 1 <= c {
                let new = (md + grid[i][j + 1]) % k;
                ans += visit(i, j + 1, grid, new, k, cache);
                ans %= 1_000_000_007;
            }
            ans %= 1_000_000_007;
            cache[i][j][md as usize] = ans;
            ans
        }
        let t = visit(0, 0, &grid, grid[0][0] % k, k, &mut cache);
        (t % 1_000_000_007) as i32
    }
}
