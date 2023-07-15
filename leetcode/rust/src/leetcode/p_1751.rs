// 1751. Maximum Number of Events That Can Be Attended II
// ------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        let mut cache: HashMap<(usize, i32), i32> = HashMap::new();

        events.sort_by(|a, b| a[0].cmp(&b[0]));

        fn binary_search(events: &Vec<Vec<i32>>, i: usize) -> usize {
            let (mut left, mut right) = (i + 1, events.len() - 1);
            let mut ans = events.len() + 1;

            while left <= right {
                let mid = (left + right) / 2;
                if events[mid][0] > events[i][1] {
                    ans = ans.min(mid);
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            ans
        }

        fn attend(
            i: usize,
            k: i32,
            events: &Vec<Vec<i32>>,
            cache: &mut HashMap<(usize, i32), i32>,
        ) -> i32 {
            if i >= events.len() || k == 0 {
                return 0;
            }
            if let Some(v) = cache.get(&(i, k)) {
                return *v;
            }
            let mut ans = 0;
            let next = binary_search(events, i);
            ans = ans.max(events[i][2] + attend(next, k - 1, events, cache));
            ans = ans.max(attend(i + 1, k, events, cache));
            cache.insert((i, k), ans);
            ans
        }

        attend(0, k, &events, &mut cache)
    }
}
