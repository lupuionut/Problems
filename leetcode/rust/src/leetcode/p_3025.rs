// 3025. Find the Number of Ways to Place People I
// -----------------------------------------------

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        /*
         *
         *  A
         *
         *       B
         * * * * * *
         */
        let mut ans = 0;
        for i in 0..points.len() {
            let ax = points[i][0];
            let ay = points[i][1];
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let bx = points[j][0];
                let by = points[j][1];

                if !(ax <= bx && by <= ay) {
                    continue;
                }
                let mut valid = true;
                for k in 0..points.len() {
                    if k == i || k == j {
                        continue;
                    }
                    let cx = points[k][0];
                    let cy = points[k][1];
                    if ((cx >= ax) && (cx <= bx)) && ((cy >= by) && (cy <= ay)) {
                        valid = false;
                        break;
                    }
                }
                if valid == true {
                    ans += 1;
                }
            }
        }
        ans
    }
}
