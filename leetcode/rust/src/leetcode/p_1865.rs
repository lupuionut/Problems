// 1865. Finding Pairs With a Certain Sum
// --------------------------------------

use std::collections::HashMap;

struct FindSumPairs {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
    counter: HashMap<i32, i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums2.len() {
            counter.entry(nums2[i]).and_modify(|v| *v += 1).or_insert(1);
        }
        Self {
            nums1,
            nums2,
            counter,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        let old = self.nums2[index as usize];
        self.counter.entry(old).and_modify(|v| *v -= 1);
        let new = old + val;
        self.nums2[index as usize] = new;
        self.counter.entry(new).and_modify(|v| *v += 1).or_insert(1);
    }

    fn count(&self, tot: i32) -> i32 {
        let mut ans = 0;
        for i in 0..self.nums1.len() {
            let val = self.nums1[i];
            let s = tot - val;
            if let Some(&v) = self.counter.get(&s) {
                ans += v;
            }
        }
        ans
    }
}
