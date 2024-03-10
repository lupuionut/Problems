// 349. Intersection of Two Arrays
// -------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut n1 = HashSet::new();
        let mut n2 = HashSet::new();

        nums1.into_iter().for_each(|n| {
            n1.insert(n);
        });
        nums2.into_iter().for_each(|n| {
            n2.insert(n);
        });

        n1.intersection(&n2).map(|&n| n).collect::<Vec<_>>()
    }
}
