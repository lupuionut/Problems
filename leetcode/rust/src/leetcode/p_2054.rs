// 2054. Two Best Non-Overlapping Events
// -------------------------------------

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MIN;
        let mut previous = 0;
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        events.sort();

        for i in 0..events.len() {
            let s = events[i][0];
            let e = events[i][1];
            let v = events[i][2];

            while heap.len() > 0 {
                if let Some(&(end, val)) = heap.peek() {
                    if -end > s {
                        break;
                    }
                }
                if let Some((end, val)) = heap.pop() {
                    previous = previous.max(val);
                }
            }
            heap.push((-e - 1, v));
            ans = ans.max(previous + v);
        }
        ans
    }
}
