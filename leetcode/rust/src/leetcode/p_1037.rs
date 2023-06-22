// 1037. Valid Boomerang
// ---------------------
// points are on same line if slopes between 2 pair of points are equal
// (y2 - y1)/(x2 - x1) == (y3 - y1)/(x3 - x1)   <=>
// (y2 - y1) * (x3 - x1) == (x2 - x1) * (y3 - y1)

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        if (points[1][1] - points[0][1]) * (points[2][0] - points[0][0])
            == (points[1][0] - points[0][0]) * (points[2][1] - points[0][1])
        {
            return false;
        }
        true
    }
}
