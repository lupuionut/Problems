// 921. Minimum Add to Make Parentheses Valid
// ------------------------------------------

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut opens = 0;
        let mut adds = 0;
        s.chars().for_each(|c| {
            if c == '(' {
                opens += 1;
            } else {
                opens -= 1;
                if opens < 0 {
                    adds += 1;
                    opens = 0;
                }
            }
        });
        if opens > 0 {
            adds += opens;
        }
        adds
    }
}
