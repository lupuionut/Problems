// 1266. Minimum Time Visiting All Points
// --------------------------------------

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 1..points.len() {
            let start = &points[i - 1];
            let end = &points[i];

            // calculate max distance that we can travel via diagonal
            let dx = (start[0] - end[0]).abs();
            let dy = (start[1] - end[1]).abs();
            let max_dist_by_diag = dx.min(dy);

            // calculate the rest of distance we can travel horiz/vert
            let rest_dist = dx.max(dy) - max_dist_by_diag;

            ans += (max_dist_by_diag + rest_dist);
        }
        ans
    }
}
