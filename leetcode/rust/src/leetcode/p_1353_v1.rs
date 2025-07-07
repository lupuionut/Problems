// 1353. Maximum Number of Events That Can Be Attended
// ---------------------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut start = BinaryHeap::new();
        let mut end = BinaryHeap::new();
        let mut used = HashSet::new();
        for i in 0..events.len() {
            let s = events[i][0];
            let e = events[i][1];
            start.push((Reverse(s), e, i));
            end.push((Reverse(e), s, i));
        }

        let mut curr = 1;
        while start.len() > 0 || end.len() > 0 {
            // remove entries with starting in the past
            while let Some(&(_, e, idx)) = start.peek() {
                if used.contains(&idx) {
                    start.pop();
                    continue;
                }
                if e < curr {
                    start.pop();
                } else {
                    break;
                }
            }

            // remove entries with ending in the past
            while let Some(&(Reverse(e), _, idx)) = end.peek() {
                if used.contains(&idx) {
                    end.pop();
                    continue;
                }
                if e < curr {
                    end.pop();
                } else {
                    break;
                }
            }
            if let Some(&(Reverse(s0), e0, i0)) = start.peek() {
                if let Some(&(Reverse(e1), s1, i1)) = end.peek() {
                    // take the one eding first
                    if curr >= s1 || s0 >= s1 {
                        end.pop();
                        if curr <= e1 {
                            used.insert(i1);
                            ans += 1;
                            if curr < s1 {
                                curr = s1 + 1;
                            } else {
                                curr += 1;
                            }
                        }
                    } else {
                        start.pop();
                        if curr <= e0 {
                            used.insert(i0);
                            ans += 1;
                        }
                        if curr < s0 {
                            curr = s0 + 1;
                        } else {
                            curr += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}
