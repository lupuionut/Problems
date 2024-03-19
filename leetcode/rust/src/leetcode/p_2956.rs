// 2956. Find Common Elements Between Two Arrays
// ---------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut n1_set = HashSet::new();
        let mut n2_set = HashSet::new();
        let mut ans = vec![0; 2];
        nums1.iter().for_each(|&n| {
            n1_set.insert(n);
        });
        nums2.iter().for_each(|&n| {
            n2_set.insert(n);
        });

        for i in 0..nums1.len() {
            if n2_set.contains(&nums1[i]) {
                ans[0] += 1;
            }
        }

        for i in 0..nums2.len() {
            if n1_set.contains(&nums2[i]) {
                ans[1] += 1;
            }
        }
        ans
    }
}
