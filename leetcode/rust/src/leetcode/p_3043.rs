// 3043. Find the Length of the Longest Common Prefix
// --------------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut first = HashSet::new();
        let mut second = HashSet::new();

        arr1.iter().for_each(|&n| {
            let mut x = n;
            first.insert(x);
            while x > 0 {
                if x > 10 {
                    first.insert(x/10);
                } else {
                    first.insert(x);
                }
                x /= 10;
            }
        });
        arr2.iter().for_each(|&n| {
            let mut x = n;
            second.insert(x);
            while x > 0 {
                if x > 10 {
                    second.insert(x/10);
                } else {
                    second.insert(x);
                }
                x /= 10;
            }
        });
        let mx = first.intersection(&second).max();
        if let Some(&x) = mx {
            let mut l = 0;
            let mut n = x;
            while n > 0 {
                l += 1;
                n /= 10;
            }
            return l;
        }
        0
    }
}
