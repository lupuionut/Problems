// 1637. Widest Vertical Area Between Two Points Containing No Points
// ------------------------------------------------------------------

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = 0;
        let mut curr = points[0][0];

        points.iter().for_each(|point| {
            ans = ans.max(point[0] - curr);
            if point[1] != 0 {
                curr = point[0];
            }
        });

        ans
    }
}
