// 2017. Grid Game
// ---------------

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let sum_top: i64 = grid[0].iter().map(|&n| n as i64).sum();
        let sum_bot: i64 = grid[1].iter().map(|&n| n as i64).sum();

        let mut best = i64::MAX;
        let mut cur_top = grid[0][0] as i64;
        let mut cur_bot = sum_bot;

        let mut score = (sum_top - cur_top).max(sum_bot - cur_bot);
        best = best.min(score);

        for i in 1..grid[0].len() {
            cur_top += grid[0][i] as i64;
            cur_bot -= grid[1][i - 1] as i64;
            score = (sum_top - cur_top).max(sum_bot - cur_bot);
            best = best.min(score);
        }

        best
    }
}
