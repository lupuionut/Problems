// 58. Length of Last Word
// -----------------------

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split(' ').filter(|w| !w.is_empty()).last().unwrap().len() as i32
    }
}
