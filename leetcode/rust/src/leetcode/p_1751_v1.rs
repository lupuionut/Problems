// 1751. Maximum Number of Events That Can Be Attended II
// ------------------------------------------------------

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort();

        fn dp(i: usize, k: i32, events: &Vec<Vec<i32>>, cache: &mut Vec<Vec<i32>>) -> i32 {
            if i == events.len() || k == 0 {
                return 0;
            }

            if cache[i][k as usize] != -1 {
                return cache[i][k as usize];
            }
            let mut take = 0;
            let nxt = next_possible(events, events[i][1]);
            if let Some(n) = nxt {
                take = events[i][2] + dp(n, k - 1, events, cache);
            } else {
                take = events[i][2];
            }
            let skip = dp(i + 1, k, events, cache);
            let best = take.max(skip);
            cache[i][k as usize] = best;
            best
        }

        fn next_possible(events: &Vec<Vec<i32>>, curr: i32) -> Option<usize> {
            let mut l = 0;
            let mut r = events.len();
            let mut g = events.len();

            while l < r {
                let mid = (l + r) / 2;
                if events[mid][0] <= curr {
                    l = mid + 1;
                } else {
                    g = g.min(mid);
                    r = mid;
                }
            }
            if g == events.len() {
                return None;
            }

            if events[g][0] > curr {
                return Some(g);
            } else {
                return None;
            }
        }
        let mut cache = vec![vec![-1; (k + 1) as usize]; events.len()];
        dp(0, k, &events, &mut cache)
    }
}
