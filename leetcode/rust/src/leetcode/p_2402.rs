// 2402. Meeting Rooms III
// -----------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let mut counter = vec![0; n as usize];
        let mut avail: BinaryHeap<i64> = BinaryHeap::new();
        let mut used: BinaryHeap<[i64; 2]> = BinaryHeap::new();

        for i in 0..n {
            avail.push(-i as i64);
        }
        meetings.sort();

        meetings.iter().for_each(|intval| {
            let start = intval[0] as i64;
            let end = intval[1] as i64;

            while used.len() > 0 {
                if let Some([time, room]) = used.pop() {
                    if -time <= start {
                        avail.push(room);
                    } else {
                        used.push([time, room]);
                        break;
                    }
                }
            }

            if avail.len() == 0 {
                if let Some([time, room]) = used.pop() {
                    counter[-room as usize] += 1;
                    let end = -time + end - start;
                    used.push([-end, room]);
                }
            } else {
                if let Some(room) = avail.pop() {
                    counter[-room as usize] += 1;
                    used.push([-end, room]);
                }
            }
        });

        let mut ans = 0;
        for i in 1..counter.len() {
            if counter[i] > counter[ans] {
                ans = i;
            }
        }

        ans as i32
    }
}
