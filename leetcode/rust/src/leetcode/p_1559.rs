// 1559. Detect Cycles in 2D Grid
// ------------------------------
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        fn visit(
            c: char,
            from: (i32, i32),
            x: i32,
            y: i32,
            grid: &Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if visited[x as usize][y as usize] == true {
                return true;
            }
            visited[x as usize][y as usize] = true;
            let diff = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut ok = false;
            for &(dx, dy) in &diff {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                    continue;
                }
                if nx == from.0 && ny == from.1 {
                    continue;
                }
                if grid[nx as usize][ny as usize] != c {
                    continue;
                }
                ok = ok || visit(c, (x, y), nx, ny, grid, visited);
            }
            false || ok
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if visited[x][y] == false {
                    let res = visit(
                        grid[x][y],
                        (-1, -1),
                        x as i32,
                        y as i32,
                        &grid,
                        &mut visited,
                    );
                    if res == true {
                        return true;
                    }
                }
            }
        }
        false
    }
}
