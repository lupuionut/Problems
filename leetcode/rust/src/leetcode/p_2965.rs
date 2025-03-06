// 2965. Find Missing and Repeated Values
// --------------------------------------

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut visited = vec![false; n * n + 1];
        let mut ans = vec![];
        for i in 0..n {
            for j in 0..n {
                let k = grid[i][j] as usize;
                if visited[k] == true {
                    ans.push(k as i32);
                } else {
                    visited[k] = true;
                }
            }
        }
        for i in 1..n * n + 1 {
            if visited[i] == false {
                ans.push(i as i32);
                break;
            }
        }

        ans
    }
}
