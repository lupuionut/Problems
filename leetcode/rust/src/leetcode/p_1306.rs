// 1306. Jump Game III
// -------------------
use std::collections::HashSet;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut q = vec![];
        let mut visited = HashSet::new();
        q.push(start as usize);

        while let Some(idx) = q.pop() {
            visited.insert(idx);
            if arr[idx] == 0 {
                return true;
            }
            let l = idx as i32 - arr[idx];
            if l >= 0 && !visited.contains(&(l as usize)) {
                q.push(l as usize);
            }
            let r = idx as i32 + arr[idx];
            if r < arr.len() as i32 && !visited.contains(&(r as usize)) {
                q.push(r as usize);
            }
        }

        false
    }
}
