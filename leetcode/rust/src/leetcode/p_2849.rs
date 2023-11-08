// 2849. Determine if a Cell Is Reachable at a Given Time
// ------------------------------------------------------

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let dx = (fx - sx).abs();
        let dy = (fy - sy).abs();

        if dx > t || dy > t {
            return false;
        }

        if t == 1 && (dx + dy) == 0 {
            return false;
        }

        true
    }
}
