// 1488. Avoid Flood in The City
// -----------------------------
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; rains.len()];
        let mut free = VecDeque::new();
        let mut full_lakes = HashMap::new();

        for i in 0..rains.len() {
            let lake = rains[i];
            if lake == 0 {
                free.push_back(i);
            } else {
                ans[i] = -1;
                if let Some(&pos) = full_lakes.get(&lake) {
                    if free.len() == 0 {
                        return vec![];
                    }
                    let mut l = 0;
                    let mut r = free.len() - 1;
                    while l < r {
                        let mid = (l + r) / 2;
                        if free[mid] > pos {
                            r = mid;
                        } else {
                            l = mid + 1;
                        }
                    }
                    if free[l] < i && free[l] > pos {
                        ans[free[l]] = lake;
                        free.remove(l);
                    } else {
                        return vec![];
                    }
                }
                full_lakes.insert(lake, i);
            }
        }
        ans
    }
}
