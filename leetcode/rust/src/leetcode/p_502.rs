// 502. IPO
// --------

use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_maximized_capital(mut k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut i0 = capital.iter();
        let mut i1 = profits.iter();
        let mut zip = i0.zip(i1);
        let mut input = vec![];
        let mut queue = VecDeque::new();
        let mut heap = BinaryHeap::new();
        let mut ans = w;

        zip.for_each(|(&c, &p)| {
            input.push((c, p));
        });

        input.sort();

        input.iter().for_each(|&p| {
            queue.push_back(p);
        });

        while k > 0 {
            while queue.len() > 0 {
                if let Some(&(c, p)) = queue.front() {
                    if c <= ans {
                        queue.pop_front();
                        heap.push(p);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if heap.len() == 0 {
                break;
            }
            let profit = heap.pop().unwrap();
            ans += profit;
            k -= 1;
        }

        ans
    }
}
