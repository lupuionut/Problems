// 389. Find the Difference
// ------------------------

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let sum_s = s.as_bytes().iter().fold(0, |acc, &b| acc + (b as i32));
        let sum_t = t.as_bytes().iter().fold(0, |acc, &b| acc + (b as i32));
        let delta = sum_t - sum_s;
        let missing = char::from_u32(delta as u32).unwrap();
        missing
    }
}
