// 1345. Jump Game IV
// ------------------
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        let mut used = HashSet::new();
        visited.insert(0);
        q.push_back((0usize, 0i32));

        arr.iter().enumerate().for_each(|(k, &v)| {
            pos.entry(v).and_modify(|p| p.push(k)).or_insert(vec![k]);
        });

        while let Some((k, jumps)) = q.pop_front() {
            if k == arr.len() - 1 {
                return jumps;
            }
            if k > 0 {
                let next = k - 1;
                if !visited.contains(&next) {
                    visited.insert(next);
                    q.push_back((next, jumps + 1));
                }
            }
            if k < arr.len() - 1 {
                let next = k + 1;
                if !visited.contains(&next) {
                    visited.insert(next);
                    q.push_back((next, jumps + 1));
                }
            }
            let val = arr[k];
            if let Some(keys) = pos.get(&val) {
                if !used.contains(&val) {
                    used.insert(val);
                    for &key in keys {
                        if !visited.contains(&key) {
                            visited.insert(key);
                            q.push_back((key, jumps + 1));
                        }
                    }
                }
            }
        }

        -1
    }
}
