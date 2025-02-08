// 2349. Design a Number Container System
// --------------------------------------

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

struct NumberContainers {
    lookup: HashMap<i32, BinaryHeap<i32>>,
    invalid: HashSet<(i32, i32)>,
    used: HashMap<i32, i32>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            lookup: HashMap::new(),
            invalid: HashSet::new(),
            used: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&n) = self.used.get(&index) {
            if n != number {
                self.invalid.insert((index, n));
            }
        }

        self.lookup
            .entry(number)
            .and_modify(|h| h.push(-index))
            .or_insert({
                let mut h = BinaryHeap::new();
                h.push(-index);
                h
            });

        if self.invalid.contains(&(index, number)) {
            self.invalid.remove(&(index, number));
        }

        self.used.insert(index, number);
    }

    fn find(&mut self, number: i32) -> i32 {
        let mut ans = -1;
        self.lookup.entry(number).and_modify(|idxs| {
            while let Some(idx) = idxs.pop() {
                if !self.invalid.contains(&(-idx, number)) {
                    ans = -idx;
                    idxs.push(idx);
                    break;
                }
            }
        });

        ans
    }
}
