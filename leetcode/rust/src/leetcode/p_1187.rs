// 1187. Make Array Strictly Increasing
// ------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        let mut cache: HashMap<(usize, i32), i32> = HashMap::new();
        arr2.sort();
        arr2.dedup();

        fn bin_search(x: i32, xs: &Vec<i32>) -> i32 {
            let mut left: i32 = 0;
            let mut right: i32 = (xs.len() - 1) as i32;

            while left <= right {
                let middle = (left + right) / 2;
                if xs[middle as usize] < x {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }

            if (left as usize) < xs.len() && xs[left as usize] != x {
                return left;
            }
            if (left as usize) < xs.len() && xs[left as usize] == x {
                return left + 1;
            }
            left
        }

        fn dp(
            i: usize,
            arr1: &Vec<i32>,
            previous: i32,
            arr2: &Vec<i32>,
            cache: &mut HashMap<(usize, i32), i32>,
        ) -> i32 {
            let mut best = 10_000;

            if i == arr1.len() {
                return 0;
            }

            if let Some(&value) = cache.get(&(i, previous)) {
                return value;
            }

            if arr1[i] > previous {
                best = best.min(dp(i + 1, arr1, arr1[i], arr2, cache));
            }

            let p = bin_search(previous, arr2) as usize;
            if p < arr2.len() {
                best = best.min(1 + dp(i + 1, arr1, arr2[p], arr2, cache));
            }

            cache.insert((i, previous), best);
            return best;
        }

        let ans = dp(0, &arr1, -1, &arr2, &mut cache);

        if ans >= 10_000 {
            return -1;
        } else {
            return ans;
        }
    }
}
