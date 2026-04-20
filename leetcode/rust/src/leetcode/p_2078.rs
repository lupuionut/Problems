// 2078. Two Furthest Houses With Different Colors
// -----------------------------------------------
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut max_right = vec![];

        for i in 0..colors.len() {
            if max_right.len() < 2 {
                max_right.push((colors[i], i));
            } else {
                if max_right[1].0 == colors[i] {
                    max_right[1].1 = i;
                } else {
                    max_right[0] = max_right[1];
                    max_right[1] = (colors[i], i);
                }
            }
        }

        let mut best = i32::MIN;
        for i in 0..colors.len() {
            if max_right[1].0 == colors[i] {
                best = best.max((max_right[0].1 - i) as i32);
            } else {
                best = best.max((max_right[1].1 - i) as i32);
            }
        }
        best
    }
}
