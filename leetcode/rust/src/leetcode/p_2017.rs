// 2017. Grid Game
// ---------------

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut p_sum_0 = vec![0; n + 1];
        let mut p_sum_1 = vec![0; n + 1];

        for i in 1..=n {
            p_sum_0[i] = p_sum_0[i - 1] + (grid[0][i - 1]) as i64;
            p_sum_1[i] = p_sum_1[i - 1] + (grid[1][i - 1]) as i64;
        }

        let mut robot_1 = std::i64::MAX;
        for i in 0..n {
            let top = p_sum_0[n] - p_sum_0[i + 1];
            let bottom = p_sum_1[i];
            let robot_2 = top.max(bottom);
            robot_1 = robot_1.min(robot_2);
        }

        robot_1
    }
}
