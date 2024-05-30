// 1442. Count Triplets That Can Form Two Arrays of Equal XOR
// ----------------------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut prefix = 0;
        let mut counter = HashMap::new();

        for j in 0..arr.len() {
            let mut next = 0;
            for k in j..arr.len() {
                next ^= arr[k];
                let key = prefix ^ next;
                if let Some(&val) = counter.get(&key) {
                    ans += val;
                }
            }
            counter.entry(prefix).and_modify(|c| *c += 1).or_insert(1);
            prefix ^= arr[j];
        }
        ans
    }
}
