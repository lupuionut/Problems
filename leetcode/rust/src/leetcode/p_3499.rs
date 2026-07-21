// 3499. Maximize Active Section with Trade I
// ------------------------------------------
impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {

        let mut groups = vec![];
        let mut prev = 0;
        let mut left = -1;
        let mut right = -1;
        s.chars().enumerate().for_each(|(k, v)| {
            if v == '0' {
                if left != -1 && right != -1 {
                    groups.push((left, right));
                }
                prev = 0;
                left = -1;
                right = -1;
            } else {
                if prev == 0 {
                    left = k as i32;
                    right = k as i32;
                } else {
                    right = k as i32;
                }
                prev = 1;
            }
        });
        if left != -1 && right != -1 {
            groups.push((left, right));
        }

        let mut best = (0, 0, -1);
        let n = (s.len() - 1) as i32;
        for i in 0..groups.len() {
            let l = groups[i].0;
            let r = groups[i].1;
            let mut d = r - l + 1;
            if l != 0 && r != n {
                if i > 0 {
                    d += l - groups[i-1].1 - 1;
                } else {
                    d += l;
                }

                if i < groups.len() - 1 {
                    d += groups[i+1].0 - r - 1;
                } else {
                    d += n - r;
                }
                if (d-r+l-1) >= best.1 {
                    best = (d, d-r+l-1, i as i32);
                }
            } 
        }

        let mut ans = 0;
        for i in 0..groups.len() {
            if i as i32 == best.2 {
                ans += best.0;
            } else {
                ans += groups[i].1 - groups[i].0 + 1;
            }
        }
        ans
    }
}
