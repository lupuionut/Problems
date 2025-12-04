// 2211. Count Collisions on a Road
// --------------------------------
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut r = 0;
        let mut l = 0;
        let mut s = 0;
        let mut ans = 0;
        directions.chars().for_each(|c| match c {
            'L' => {
                l += 1;
                if s != 0 {
                    ans += l;
                    l = 0;
                    s = 1;
                    r = 0;
                } else if r != 0 {
                    ans += 2;
                    ans += 0.max(r - 1);
                    ans += 0.max(l - 1);
                    s = 1;
                    l = 0;
                    r = 0;
                } else {
                    l = 0;
                    r = 0;
                    s = 0;
                }
            }
            'R' => {
                s = 0;
                l = 0;
                r += 1;
            }
            _ => {
                if r != 0 {
                    ans += r;
                    r = 0;
                    l = 0;
                }
                s = 1;
            }
        });
        ans
    }
}
