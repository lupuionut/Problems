// 1344. Angle Between Hands of a Clock
// ------------------------------------
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mut min_delta_procent = minutes as f64 / 60.0;
        let mut hour_delta_sec = 300.0 * min_delta_procent;
        let mut hour_sec = 0f64;
        if hour == 12 {
            hour_sec += hour_delta_sec;
        } else {
            hour_sec = (hour as f64 * 5.0) * 60.0 + hour_delta_sec;
        }
        let min_sec = minutes as f64 * 60.0;
        let degree = 360.0 / 3600.0;
        let a = (min_sec - hour_sec).abs() * degree;
        let b = 360.0 - a;
        a.min(b)
    }
}
