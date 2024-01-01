// 455. Assign Cookies
// -------------------

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut ans = 0;
        g.sort();
        s.sort();

        let mut j = 0;
        let mut i = 0;

        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                ans += 1;
                i += 1;
            }
            j += 1;
        }

        ans
    }
}
