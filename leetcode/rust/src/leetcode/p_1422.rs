// 1422. Maximum Score After Splitting a String
// --------------------------------------------

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let n = s.len();
        let mut ans = 0;
        let mut ps = vec![(0, 0); n + 1];

        s.as_bytes().iter().enumerate().for_each(|(i, c)| {
            let p = match c {
                b'0' => (ps[i].0 + 1, ps[i].1),
                b'1' => (ps[i].0, ps[i].1 + 1),
                _ => ps[i],
            };
            ps[i + 1] = p;
        });

        for i in 1..n {
            let score = ps[i].0 + (ps[n].1 - ps[i].1);
            ans = ans.max(score);
        }

        ans
    }
}
