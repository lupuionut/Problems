// 2402. Meeting Rooms III
// -----------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort();
        let mut available = BinaryHeap::new();
        let mut maybe = BinaryHeap::new();
        let mut counter = vec![0; n as usize];
        for i in 0..n as usize {
            available.push((Reverse(i), 0));
        }
        for meeting in &meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;

            while maybe.len() > 0 {
                if let Some(&(Reverse(time), Reverse(room))) = maybe.peek() {
                    if time <= start {
                        available.push((Reverse(room), time));
                        maybe.pop();
                    } else {
                        break;
                    }
                }
            }

            if let Some((Reverse(room), time)) = available.pop() {
                counter[room] += 1;
                if time <= start {
                    maybe.push((Reverse(end), Reverse(room)))
                } else {
                    let delta = end - start;
                    maybe.push((Reverse(time + delta), Reverse(room)));
                }
            } else if let Some((Reverse(time), Reverse(room))) = maybe.pop() {
                counter[room] += 1;
                if time <= start {
                    maybe.push((Reverse(end), Reverse(room)));
                } else {
                    let delta = end - start;
                    maybe.push((Reverse(time + delta), Reverse(room)));
                }
            }
        }

        let mut ans = 0;
        for i in 0..n as usize {
            if counter[i] > counter[ans] {
                ans = i;
            }
        }
        ans as i32
    }
}
