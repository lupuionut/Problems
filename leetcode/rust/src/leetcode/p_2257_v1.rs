// 2257. Count Unguarded Cells in the Grid
// ---------------------------------------
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec!['f'; n as usize]; m as usize];
        let mut rows = vec![false; m as usize];
        let mut cols = vec![false; n as usize];
        let mut ans = m * n;

        for guard in guards {
            grid[guard[0] as usize][guard[1] as usize] = 'g';
            ans -= 1;
        }
        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = 'w';
            ans -= 1;
        }

        for row in 0..m as usize {
            for col in 0..n as usize {
                if grid[row][col] == 'w' {
                    rows[row] = false;
                    cols[col] = false;
                } else if grid[row][col] == 'g' {
                    rows[row] = true;
                    cols[col] = true;
                } else {
                    if rows[row] == true || cols[col] == true {
                        grid[row][col] = 'x';
                        ans -= 1;
                    }
                }
            }
        }

        rows = vec![false; m as usize];
        cols = vec![false; n as usize];
        for row in (0..m as usize).rev() {
            for col in (0..n as usize).rev() {
                if grid[row][col] == 'w' {
                    rows[row] = false;
                    cols[col] = false;
                } else if grid[row][col] == 'g' {
                    rows[row] = true;
                    cols[col] = true;
                } else {
                    if rows[row] == true || cols[col] == true {
                        if grid[row][col] != 'x' {
                            ans -= 1;
                        }
                    }
                }
            }
        }

        ans
    }
}
