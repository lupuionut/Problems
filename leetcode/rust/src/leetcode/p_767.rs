// 767. Reorganize String
// ----------------------

use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut bs = vec![];
        let mut last = b'0';

        s.as_bytes().iter().for_each(|c| {
            map.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        for (&k, &v) in map.iter() {
            heap.push((v, k));
        }

        while heap.len() > 0 {
            let z = heap.pop().unwrap();
            // if it's not the same char
            if *z.1 != last {
                bs.push(*z.1);
                if z.0 > 1 {
                    heap.push((z.0 - 1, z.1));
                }
                last = *z.1;
            } else {
                if heap.len() == 0 {
                    return "".to_string();
                }
                let y = heap.pop().unwrap();
                bs.push(*y.1);
                if y.0 > 1 {
                    heap.push((y.0 - 1, y.1));
                }
                heap.push(z);
                last = *y.1;
            }
        }
        String::from_utf8(bs).unwrap()
    }
}
