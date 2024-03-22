// 2486. Append Characters to String to Make Subsequence
// -----------------------------------------------------

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut s = s.chars();
        let mut t = t.chars();
        let mut curr_t = t.next();
        let mut curr_s = s.next();

        while curr_s.is_some() {
            if curr_t == curr_s {
                curr_t = t.next();
                curr_s = s.next();
            } else {
                curr_s = s.next();
            }
        }

        if curr_t.is_some() {
            return t.count() as i32 + 1;
        }
        0
    }
}
