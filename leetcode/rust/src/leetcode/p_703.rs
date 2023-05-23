// 703. Kth Largest Element in a Stream

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    nums: BinaryHeap<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut ns: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for i in nums {
            ns.push(Reverse(i));
        }
        return KthLargest { nums: ns, k };
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(Reverse(val));

        loop {
            if self.k == self.nums.len() as i32 {
                break;
            }
            self.nums.pop();
        }
        if let Some(Reverse(ans)) = self.nums.peek() {
            *ans
        } else {
            0
        }
    }
}
