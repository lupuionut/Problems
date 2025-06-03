// 1298. Maximum Candies You Can Get from Boxes
// --------------------------------------------

use std::collections::HashSet;
impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut avail_keys = HashSet::new();
        let mut avail_boxes = HashSet::new();
        let mut q = vec![];
        for i in 0..initial_boxes.len() {
            let k = initial_boxes[i];
            if status[k as usize] == 1 {
                q.push(k);
            } else {
                avail_boxes.insert(k);
            }
        }
        let mut ans = 0;

        while q.len() > 0 {
            if let Some(curr) = q.pop() {
                ans += candies[curr as usize];
                for &key in &keys[curr as usize] {
                    if avail_boxes.contains(&key) {
                        q.push(key);
                        avail_boxes.remove(&key);
                    } else {
                        avail_keys.insert(key);
                    }
                }
                for &bx in &contained_boxes[curr as usize] {
                    if status[bx as usize] == 1 {
                        q.push(bx);
                    } else {
                        if avail_keys.contains(&bx) {
                            q.push(bx);
                            avail_keys.remove(&bx);
                        } else {
                            avail_boxes.insert(bx);
                        }
                    }
                }
            }
        }

        ans
    }
}
