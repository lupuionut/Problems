// 1828. Queries on Number of Points Inside a Circle
// -------------------------------------------------

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        fn inside_circle(query: &Vec<i32>, x: i32, y: i32) -> bool {
            if let [cx, cy, cr] = &query[..] {
                let c1 = (cx - x);
                let c2 = (cy - y);
                if (c1 * c1) + (c2 * c2) <= (cr * cr) {
                    return true;
                }
            }
            false
        }

        for query in queries {
            let mut a: i32 = 0;
            points.iter().for_each(|point| {
                if inside_circle(&query, point[0], point[1]) {
                    a += 1;
                }
            });
            ans.push(a);
        }

        ans
    }
}
