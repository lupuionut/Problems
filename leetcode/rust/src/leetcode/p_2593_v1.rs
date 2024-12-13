// 2593. Find Score of an Array After Marking All Elements
// -------------------------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut visited = vec![false; nums.len()];
        let mut heap = BinaryHeap::new();
        let mut ans = 0;

        nums.iter().enumerate().for_each(|(k, &v)| {
            heap.push((-v, -(k as i32)));
        });

        while heap.len() > 0 {
            if let Some((v, idx)) = heap.pop() {
                let k = (-idx) as usize;
                if visited[k] == true {
                    continue;
                } else {
                    ans += -v as i64;
                    visited[k] = true;
                    if k > 0 {
                        visited[k - 1] = true;
                    }
                    if k < nums.len() - 1 {
                        visited[k + 1] = true;
                    }
                }
            }
        }

        ans
    }
}
