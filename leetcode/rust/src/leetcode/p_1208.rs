// 1208. Get Equal Substrings Within Budget
// ----------------------------------------

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut l = 0;
        let mut r = 0;
        let mut window_cost = 0;
        let mut ans = 0;

        while r < s.len() {
            let diff = if s[r] > t[r] {
                s[r] - t[r]
            } else {
                t[r] - s[r]
            };

            window_cost += diff as i32;
            while window_cost > max_cost {
                let diff = if s[l] > t[l] {
                    s[l] - t[l]
                } else {
                    t[l] - s[l]
                };
                window_cost -= diff as i32;
                l += 1;
            }

            ans = ans.max((r - l + 1) as i32);
            r += 1;
        }

        ans
    }
}
