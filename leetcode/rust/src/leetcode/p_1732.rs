// 1732. Find the Highest Altitude
// -------------------------------

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut acc = 0;
        for i in 0..gain.len() {
            acc += gain[i];
            if max < acc {
                max = acc;
            }
        }
        max
    }
}
