// 1376. Time Needed to Inform All Employees
// -----------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut time = 0;
        let mut subord: Vec<Vec<i32>> = vec![vec![]; n as usize];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        for i in 0..n {
            if manager[i as usize] == -1 {
                continue;
            }
            subord[manager[i as usize] as usize].push(i);
        }

        queue.push_back((head_id, 0));
        while queue.len() > 0 {
            let (node, t) = queue.pop_front().unwrap();
            if t > time {
                time = t;
            }
            for sub in &subord[node as usize] {
                queue.push_back((*sub, t + inform_time[node as usize]));
            }
        }

        time
    }
}
