// 3394. Check if Grid can be Cut into Sections
// --------------------------------------------

impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let mut ys = vec![[0; 2]; rectangles.len()];
        let mut xs = vec![[0; 2]; rectangles.len()];

        for i in 0..rectangles.len() {
            ys[i][0] = rectangles[i][1];
            ys[i][1] = rectangles[i][3];
            xs[i][0] = rectangles[i][0];
            xs[i][1] = rectangles[i][2];
        }

        xs.sort();
        ys.sort();

        let mut count = 0;
        let mut end = -1;

        for i in 0..xs.len() {
            let s = xs[i][0];
            let e = xs[i][1];
            if s >= end && end != -1 {
                count += 1;
            }
            end = end.max(e);
        }
        if count >= 2 {
            return true;
        }

        count = 0;
        end = -1;
        for i in 0..ys.len() {
            let s = ys[i][0];
            let e = ys[i][1];
            if s >= end && end != -1 {
                count += 1;
            }
            end = end.max(e);
        }
        if count >= 2 {
            return true;
        }

        false
    }
}
