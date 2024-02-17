// 1642. Furthest Building You Can Reach
// -------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut total_used_bricks = 0;
        let mut total_used_times = 0;
        let mut used_bricks = BinaryHeap::new();

        for i in 1..heights.len() {
            let diff = heights[i] - heights[i - 1];
            if diff > 0 {
                total_used_times += 1;
                total_used_bricks += diff;
                used_bricks.push(diff);
                while used_bricks.len() > 0 && total_used_bricks > bricks {
                    let val = used_bricks.pop().unwrap();
                    total_used_bricks -= val;
                }
                if used_bricks.len() as i32 + ladders < total_used_times {
                    return (i - 1) as i32;
                }
            }
        }
        heights.len() as i32 - 1
    }
}
