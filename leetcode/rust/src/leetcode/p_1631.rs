// 1631. Path With Minimum Effort
// ------------------------------

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = 1_000_001;

        fn can_pass(curr: i32, heights: &Vec<Vec<i32>>) -> bool {
            let mut queue = vec![];
            let m = heights.len() - 1;
            let n = heights[0].len() - 1;
            let mut visited = vec![vec![false; (n + 1) as usize]; (m + 1) as usize];

            queue.push((0, 0));
            visited[0][0] = true;

            while queue.len() > 0 {
                let (i, j) = queue.pop().unwrap();
                if i == m && j == n {
                    return true;
                }
                let d = &[(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
                for &(ni, nj) in d {
                    if ni >= 0 && ni <= m && nj >= 0 && nj <= n {
                        let effort = (heights[i as usize][j as usize]
                            - heights[ni as usize][nj as usize])
                            .abs();
                        if effort <= curr && visited[ni as usize][nj as usize] == false {
                            visited[ni as usize][nj as usize] = true;
                            queue.push((ni, nj));
                        }
                    }
                }
            }

            false
        }

        while left <= right {
            let middle = (left + right) / 2;
            if can_pass(middle, &heights) {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        left
    }
}
