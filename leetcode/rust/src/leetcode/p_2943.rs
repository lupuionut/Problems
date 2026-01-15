// 2943. Maximize Area of Square Hole in Grid
// ------------------------------------------
impl Solution {
    pub fn maximize_square_hole_area(
        n: i32,
        m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort();
        v_bars.sort();

        let mut best_h = 1;
        let mut best_v = 1;
        let mut h_streak = 1;
        let mut v_streak = 1;
        for i in 1..h_bars.len() {
            if h_bars[i] == h_bars[i - 1] + 1 {
                h_streak += 1;
            } else {
                h_streak = 1;
            }
            best_h = best_h.max(h_streak);
        }
        for i in 1..v_bars.len() {
            if v_bars[i] == v_bars[i - 1] + 1 {
                v_streak += 1;
            } else {
                v_streak = 1;
            }
            best_v = best_v.max(v_streak);
        }
        let l = best_h.min(best_v) + 1;
        l * l
    }
}
