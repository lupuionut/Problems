// 1905. Count Sub Islands
// -----------------------

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut grid2 = grid2;
        let m = grid2.len();
        let n = grid2[0].len();
        let mut counter = 0;

        fn is_subisland(grid2: &mut Vec<Vec<i32>>, grid1: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
            let mut queue = vec![(i, j)];
            let mut ans = true;

            while queue.len() > 0 {
                let (i, j) = queue.pop().unwrap();

                if grid1[i as usize][j as usize] == 0 {
                    ans = false;
                }

                grid2[i as usize][j as usize] = 2;

                let d = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                for (dx, dy) in d {
                    if i + dx < 0 || i + dx >= grid2.len() as i32 {
                        continue;
                    }
                    if j + dy < 0 || j + dy >= grid2[0].len() as i32 {
                        continue;
                    }
                    if grid2[(i + dx) as usize][(j + dy) as usize] == 1 {
                        queue.push((i + dx, j + dy));
                    }
                }
            }

            ans
        }

        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 {
                    if is_subisland(&mut grid2, &grid1, i as i32, j as i32) == true {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }
}
