// 898. Bitwise ORs of Subarrays
// -----------------------------

use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut unique = HashSet::new();
        let mut newarr = vec![];
        let mut curr = -1;
        arr.iter().for_each(|&x| {
            if x != curr {
                newarr.push(x);
            }
            unique.insert(x);
            curr = x;
        });

        let mut subarr = HashSet::new();

        for i in 0..newarr.len() {
            let curr = newarr[i];
            let mut t = HashSet::new();
            t.insert(curr);
            for val in subarr.iter() {
                let nv = newarr[i] | val;
                unique.insert(nv);
                t.insert(nv);
            }
            subarr = t;
        }

        unique.len() as i32
    }
}
