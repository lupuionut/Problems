// 373. Find K Pairs with Smallest Sums
// ------------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut k = k;
        let mut heap = BinaryHeap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut ans: Vec<Vec<i32>> = vec![];
        visited.insert((0, 0));
        heap.push((Reverse(nums1[0] + nums2[0]), 0, 0));

        while heap.len() > 0 && k > 0 {
            let pair = heap.pop().unwrap();
            ans.push(vec![nums1[pair.1], nums2[pair.2]]);

            let i = pair.1 + 1;
            if i < nums1.len() && visited.contains(&(i, pair.2)) == false {
                heap.push((Reverse(nums1[i] + nums2[pair.2]), i, pair.2));
                visited.insert((i, pair.2));
            }

            let j = pair.2 + 1;
            if j < nums2.len() && visited.contains(&(pair.1, j)) == false {
                heap.push((Reverse(nums1[pair.1] + nums2[j]), pair.1, j));
                visited.insert((pair.1, j));
            }

            k -= 1;
        }
        ans
    }
}
