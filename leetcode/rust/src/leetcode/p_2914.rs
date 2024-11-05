// 2914. Minimum Number of Changes to Make Binary String Beautiful
// ---------------------------------------------------------------

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut curr = (None, 0);
        let mut ans = 0;
        s.chars().for_each(|cr| {
            if let Some(c) = curr.0 {
                if c != cr {
                    if curr.1 % 2 != 0 {
                        ans += 1;
                        curr = (None, 0);
                    } else {
                        curr = (Some(cr), 1);
                    }
                } else {
                    curr.1 += 1;
                }
            } else {
                curr = (Some(cr), 1);
            }
        });
        ans
    }
}
