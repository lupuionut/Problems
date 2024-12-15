// 1792. Maximum Average Pass Ratio
// --------------------------------

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();
        let mut used = 0;

        for class in &classes {
            let r = (class[0] as f64 + 1.0) / (class[1] as f64 + 1.0)
                - (class[0] as f64) / (class[1] as f64);
            heap.push((OrderedFloat(r), class[0], class[1]))
        }
        while used < extra_students {
            if let Some((_, p, t)) = heap.pop() {
                let p = p + 1;
                let t = t + 1;
                let r = (p as f64 + 1.0) / (t as f64 + 1.0) - (p as f64 / t as f64);
                heap.push((OrderedFloat(r), p, t))
            }
            used += 1;
        }
        let mut ans = 0_f64;
        for &(_, p, t) in heap.iter() {
            ans += (p as f64 / t as f64);
        }
        ans / classes.len() as f64
    }
}
