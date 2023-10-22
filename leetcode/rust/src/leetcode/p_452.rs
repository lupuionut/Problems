// 452. Minimum Number of Arrows to Burst Balloons
// -----------------------------------------------
// Order by the end of each point
// each time a new point starts after current end
// increase the number of arrows

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut current: Option<Vec<i32>> = None;
        let mut ans = 0;

        points.iter().for_each(|p| {
            if current.is_none() || p[0] > current.as_ref().unwrap()[1] {
                ans += 1;
                current = Some(p.to_vec());
            }
        });

        ans
    }
}
