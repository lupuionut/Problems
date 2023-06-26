// 2462. Total Cost to Hire K Workers
// ----------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut k = k;
        let mut ans = 0;
        let mut heap_left = BinaryHeap::new();
        let mut heap_right = BinaryHeap::new();
        let mut rest = VecDeque::new();
        let mut fs = 0;
        let mut fe = candidates as usize;
        let mut ss = costs.len() - candidates as usize;
        let mut se = costs.len();

        if 2 * candidates as usize > costs.len() {
            fe = costs.len() / 2;
            ss = fe;
        }

        for i in fs..fe as usize {
            let pair = Reverse(costs[i]);
            heap_left.push(pair);
        }

        for i in (ss..se).rev() {
            let pair = Reverse(costs[i]);
            heap_right.push(pair);
        }

        for c in &costs[fe..ss] {
            rest.push_back(c);
        }

        while k > 0 {
            let result = (heap_left.peek(), heap_right.peek());
            match result {
                (None, None) => {}
                (None, Some(_)) => {
                    let Reverse(a) = heap_right.pop().unwrap();
                    ans += a as i64;
                    if rest.len() > 0 {
                        let val = rest.pop_back().unwrap();
                        heap_right.push(Reverse(*val));
                    }
                }
                (Some(Reverse(a)), Some(Reverse(b))) => {
                    if a <= b {
                        let Reverse(a) = heap_left.pop().unwrap();
                        ans += a as i64;
                        if rest.len() > 0 {
                            let val = rest.pop_front().unwrap();
                            heap_left.push(Reverse(*val));
                        }
                    } else {
                        let Reverse(a) = heap_right.pop().unwrap();
                        ans += a as i64;
                        if rest.len() > 0 {
                            let val = rest.pop_back().unwrap();
                            heap_right.push(Reverse(*val));
                        }
                    }
                }
                (Some(_), None) => {
                    let Reverse(a) = heap_left.pop().unwrap();
                    ans += a as i64;
                    if rest.len() > 0 {
                        let val = rest.pop_front().unwrap();
                        heap_left.push(Reverse(*val));
                    }
                }
            }
            k -= 1;
        }

        ans
    }
}
