// 2833. Furthest Point From Origin
// --------------------------------
impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut lr = (0, 0);
        moves.chars().for_each(|c| {
            if c == 'L' {
                lr.0 += 1;
                lr.1 -= 1;
            } else if c == 'R' {
                lr.1 += 1;
                lr.0 -= 1;
            } else {
                lr.0 += 1;
                lr.1 += 1;
            }
        });

        lr.0.max(lr.1)
    }
}
