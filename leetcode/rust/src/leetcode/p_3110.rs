// 3110. Score of a String
// -----------------------

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans = 0;
        s.as_bytes().windows(2).for_each(|intv| {
            ans += (intv[0] as i32 - intv[1] as i32).abs();
        });
        ans
    }
}
