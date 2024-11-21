// 2257. Count Unguarded Cells in the Grid
// ---------------------------------------

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut prison = vec![vec![0; n as usize]; m as usize];
        let mut visited = vec![vec![(false, false); n as usize]; m as usize];
        let mut ans = 0;

        walls.iter().for_each(|wall| {
            let row = wall[0] as usize;
            let col = wall[1] as usize;
            prison[row][col] = -1;
        });
        guards.iter().for_each(|guard| {
            let row = guard[0] as usize;
            let col = guard[1] as usize;
            Self::visit(row as i32, col as i32, &mut prison, &mut visited);
        });

        for i in 0..prison.len() {
            for j in 0..prison[0].len() {
                if prison[i][j] == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }

    pub fn visit(
        row: i32,
        col: i32,
        prison: &mut Vec<Vec<i32>>,
        visited: &mut Vec<Vec<(bool, bool)>>,
    ) {
        prison[row as usize][col as usize] = 1;

        let mut r = row - 1;
        while r >= 0 {
            if prison[r as usize][col as usize] == -1 {
                break;
            }
            if visited[r as usize][col as usize].0 == true {
                break;
            }
            visited[r as usize][col as usize].0 = true;
            prison[r as usize][col as usize] = 1;
            r -= 1;
        }
        let mut r = row + 1;
        while r < prison.len() as i32 {
            if prison[r as usize][col as usize] == -1 {
                break;
            }
            if visited[r as usize][col as usize].0 == true {
                break;
            }
            visited[r as usize][col as usize].0 = true;
            prison[r as usize][col as usize] = 1;
            r += 1;
        }
        let mut c = col - 1;
        while c >= 0 {
            if prison[row as usize][c as usize] == -1 {
                break;
            }
            if visited[row as usize][c as usize].1 == true {
                break;
            }
            visited[row as usize][c as usize].1 = true;
            prison[row as usize][c as usize] = 1;
            c -= 1;
        }
        let mut c = col + 1;
        while c < prison[0].len() as i32 {
            if prison[row as usize][c as usize] == -1 {
                break;
            }
            if visited[row as usize][c as usize].1 == true {
                break;
            }
            visited[row as usize][c as usize].1 = true;
            prison[row as usize][c as usize] = 1;
            c += 1;
        }
    }
}
