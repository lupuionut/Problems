// 3362. Zero Array Transformation III
// -----------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let mut available: BinaryHeap<i32> = BinaryHeap::new();
        let mut using: BinaryHeap<i32> = BinaryHeap::new();
        let total = queries.len() as i32;

        queries.sort();
        queries = queries.into_iter().rev().collect();

        let mut count = 0;
        for (key, &num) in nums.iter().enumerate() {
            while queries.len() > 0 {
                if let Some(itval) = queries.pop() {
                    if itval[0] > key as i32 {
                        queries.push(itval);
                        break;
                    }
                    available.push(itval[1]);
                }
            }

            while using.len() > 0 && -*using.peek().unwrap() < key as i32 {
                using.pop();
            }

            while (using.len() as i32) < num {
                if available.len() == 0 {
                    return -1;
                }
                if let Some(n) = available.pop() {
                    if n < key as i32 {
                        continue;
                    }
                    using.push(-n);
                    count += 1;
                }
            }
        }

        total - count
    }
}
