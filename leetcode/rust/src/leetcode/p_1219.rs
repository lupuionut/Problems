// 1219. Path with Maximum Gold
// ----------------------------

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        fn get_best(i: i32, j: i32, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
            let next = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
            let mut best = grid[i as usize][j as usize];
            visited[i as usize][j as usize] = true;
            for (dx, dy) in next {
                if dx >= 0
                    && dx < grid.len() as i32
                    && dy >= 0
                    && dy < grid[0].len() as i32
                    && grid[dx as usize][dy as usize] != 0
                    && visited[dx as usize][dy as usize] == false
                {
                    best = best.max(grid[i as usize][j as usize] + get_best(dx, dy, grid, visited));
                }
            }
            visited[i as usize][j as usize] = false;
            best
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
                    ans = ans.max(get_best(i as i32, j as i32, &grid, &mut visited));
                }
            }
        }

        ans
    }
}
