// 789. Escape The Ghosts
// ----------------------

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let distance = target[0].abs() + target[1].abs();
        for point in ghosts.iter() {
            let dist = (target[0] - point[0]).abs() + (target[1] - point[1]).abs();
            if dist <= distance {
                return false;
            }
        }
        true
    }
}
