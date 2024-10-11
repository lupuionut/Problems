// 1942. The Number of the Smallest Unoccupied Chair
// -------------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut events: Vec<(i32, i32, i32)> = vec![];
        let mut available_chairs = BinaryHeap::new();
        let mut used_chairs: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

        for i in 0..times.len() {
            available_chairs.push(Reverse(i as i32));
        }

        times.iter().enumerate().for_each(|(k, v)| {
            events.push((v[0], v[1], k as i32));
        });
        events.sort();

        for event in events {
            let start = event.0;
            let end = event.1;
            let index = event.2;

            while used_chairs.len() > 0 {
                if let Some(Reverse(used)) = used_chairs.pop() {
                    if used.0 <= start {
                        available_chairs.push(Reverse(used.1));
                    } else {
                        used_chairs.push(Reverse(used));
                        break;
                    }
                } else {
                    break;
                }
            }

            if let Some(Reverse(chair)) = available_chairs.pop() {
                used_chairs.push(Reverse((end, chair)));

                if index == target_friend {
                    return chair;
                }
            }
        }

        -1
    }
}
