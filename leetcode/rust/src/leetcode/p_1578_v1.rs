// 1578. Minimum Time to Make Rope Colorful
// ----------------------------------------
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last = ('d', 0);
        let chars: Vec<char> = colors.chars().collect();
        for i in 0..chars.len() {
            if last.0 == chars[i] {
                if last.1 < needed_time[i] {
                    ans += last.1;
                    last.1 = needed_time[i];
                } else {
                    ans += needed_time[i];
                }
            } else {
                last = (chars[i], needed_time[i]);
            }
        }
        ans
    }
}
