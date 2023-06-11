// 657. Robot Return to Origin
// ---------------------------

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut position = (0, 0);

        for c in moves.chars() {
            match c {
                'U' => position.1 += 1,
                'D' => position.1 -= 1,
                'L' => position.0 -= 1,
                'R' => position.0 += 1,
                _ => {}
            }
        }

        if position == (0, 0) {
            true
        } else {
            false
        }
    }
}
