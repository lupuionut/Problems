// 2825. Make String a Subsequence Using Cyclic Increments
// -------------------------------------------------------

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let chars1 = str1.chars().collect::<Vec<_>>();
        let chars2 = str2.chars().collect::<Vec<_>>();
        let mut j = 0;
        for i in 0..chars1.len() {
            if j == chars2.len() {
                return true;
            }
            if chars1[i] == chars2[j] {
                j += 1;
                continue;
            }
            if Self::next_char(chars1[i]) == chars2[j] {
                j += 1;
                continue;
            }
        }
        if j == chars2.len() {
            true
        } else {
            false
        }
    }

    pub fn next_char(c: char) -> char {
        let mut i = (c as i32) - 97;
        i = 97 + ((i + 1) % 26);
        char::from_u32(i as u32).unwrap()
    }
}
