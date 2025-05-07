// 3341. Find Minimum Time to Reach Last Room I
// --------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let mut queue = BinaryHeap::new();
        let m = move_time[0].len();
        let n = move_time.len();
        let mut visited = vec![vec![false; m]; n];
        queue.push(Reverse((0, (0i32, 0i32))));

        while queue.len() > 0 {
            if let Some(Reverse((curr_time, (x, y)))) = queue.pop() {
                if visited[x as usize][y as usize] {
                    continue;
                }
                visited[x as usize][y as usize] = true;

                if x == (n - 1) as i32 && y == (m - 1) as i32 {
                    return curr_time;
                }
                for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0
                        || nx >= n as i32
                        || ny < 0
                        || ny >= m as i32
                        || visited[nx as usize][ny as usize] == true
                    {
                        continue;
                    }
                    let new_time = curr_time.max(move_time[nx as usize][ny as usize]) + 1;
                    queue.push(Reverse((new_time, (nx, ny))));
                }
            }
        }

        -1
    }
}
