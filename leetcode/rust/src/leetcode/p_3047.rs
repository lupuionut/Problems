// 3047. Find the Largest Area of Square Inside Two Rectangles
// -----------------------------------------------------------
impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut coords = vec![[(0, 0), (0, 0)]];
        for i in 0..bottom_left.len() {
            coords.push([
                (bottom_left[i][0], bottom_left[i][1]),
                (top_right[i][0], top_right[i][1]),
            ]);
        }
        coords.sort();
        let mut best = 0;
        for i in 0..coords.len() {
            let [(x0, y0), (x1, y1)] = coords[i];
            for j in (i + 1)..coords.len() {
                let [(x2, y2), (x3, y3)] = coords[j];
                let ix = intersect(x0, x1, x2, x3);
                let iy = intersect(y0, y1, y2, y3);
                let l = ix.min(iy) as i64;
                best = best.max(l * l);
            }
        }

        fn intersect(x0: i32, x1: i32, x2: i32, x3: i32) -> i32 {
            0.max(x1.min(x3) - x0.max(x2))
        }

        best
    }
}
