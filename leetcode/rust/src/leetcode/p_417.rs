// 417. Pacific Atlantic Water Flow
// --------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();
        let rows = heights.len();
        let cols = heights[0].len();

        let mut queue = vec![];
        let mut visited = vec![vec![false; cols]; rows];
        for i in 0..cols {
            queue.push((0, i as i32));
        }
        for i in 0..rows {
            queue.push((i as i32, 0));
        }

        while queue.len() > 0 {
            let (x, y) = queue.pop().unwrap();
            pacific.insert(vec![x, y]);
            visited[x as usize][y as usize] = true;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0
                    || nx >= rows as i32
                    || ny < 0
                    || ny >= cols as i32
                    || visited[nx as usize][ny as usize] == true
                {
                    continue;
                }
                if heights[nx as usize][ny as usize] >= heights[x as usize][y as usize] {
                    queue.push((nx, ny));
                }
            }
        }

        visited = vec![vec![false; cols]; rows];
        for i in 0..cols {
            queue.push(((rows - 1) as i32, i as i32));
        }
        for i in 0..rows {
            queue.push((i as i32, (cols - 1) as i32));
        }
        while queue.len() > 0 {
            let (x, y) = queue.pop().unwrap();
            atlantic.insert(vec![x, y]);
            visited[x as usize][y as usize] = true;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0
                    || nx >= rows as i32
                    || ny < 0
                    || ny >= cols as i32
                    || visited[nx as usize][ny as usize] == true
                {
                    continue;
                }
                if heights[nx as usize][ny as usize] >= heights[x as usize][y as usize] {
                    queue.push((nx, ny));
                }
            }
        }

        atlantic.intersection(&pacific).map(|v| v.clone()).collect()
    }
}
