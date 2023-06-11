// 1200. Minimum Absolute Difference
// ---------------------------------

use std::i32::MAX;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut difference = i32::MAX;
        let mut arr = arr;
        arr.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut iter = arr.windows(2);

        while let Some(pair) = iter.next() {
            if pair[1] - pair[0] < difference {
                difference = pair[1] - pair[0];
            }
        }

        let mut iter = arr.windows(2);
        while let Some(pair) = iter.next() {
            if pair[1] - pair[0] == difference {
                ans.push(pair.to_vec());
            }
        }

        ans
    }
}
