// 2033. Minimum Operations to Make a Uni-Value Grid
// -------------------------------------------------

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut ns = vec![];
        let mut ans = 0;
        let first = grid[0][0] % x;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] % x != first {
                    return -1;
                }
                ns.push(grid[i][j]);
            }
        }

        ns.sort();
        let mid = ns.len() / 2;
        let median = ns[mid];

        for i in 0..ns.len() {
            if i < mid {
                ans += (median - ns[i]) / x;
            } else {
                ans += (ns[i] - median) / x;
            }
        }

        ans
    }
}
