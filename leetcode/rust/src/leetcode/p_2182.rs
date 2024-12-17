// 2182. Construct String With Repeat Limit
// ----------------------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut letters = vec![0; 26];
        let mut heap = BinaryHeap::new();
        let mut ans = vec![];

        s.chars().for_each(|c| {
            let idx = (c as i32 - 97) as usize;
            letters[idx] += 1;
        });

        for i in 0..26 {
            if letters[i] != 0 {
                let k = (97 + i) as u32;
                let c = char::from_u32(k).unwrap();
                heap.push((c, letters[i]));
            }
        }
        let mut curr = None;
        let mut streak = 0;

        while heap.len() > 0 {
            if let Some(head) = heap.pop() {
                if streak == repeat_limit {
                    if curr == Some(head.0) {
                        if let Some(next) = heap.pop() {
                            curr = Some(next.0);
                            ans.push(next.0);
                            if next.1 > 1 {
                                heap.push((next.0, next.1 - 1));
                            }
                            heap.push(head);
                        } else {
                            break;
                        }
                    } else {
                        curr = Some(head.0);
                        ans.push(head.0);
                        if head.1 > 1 {
                            heap.push((head.0, head.1 - 1));
                        }
                    }
                    streak = 1;
                } else {
                    if curr == Some(head.0) {
                        streak += 1;
                    } else {
                        streak = 1;
                    }
                    curr = Some(head.0);
                    ans.push(head.0);
                    if head.1 > 1 {
                        heap.push((head.0, head.1 - 1));
                    }
                }
            }
        }

        ans.iter().collect::<String>()
    }
}
