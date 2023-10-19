// 844. Backspace String Compare
// -----------------------------

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_i = s.len() as i32 - 1;
        let mut t_i = t.len() as i32 - 1;
        let mut s_back = 0;
        let mut t_back = 0;
        let s = s.as_str();
        let t = t.as_str();

        while s_i >= 0 || t_i >= 0 {
            let mut s_char = None;
            let mut t_char = None;

            while s_char.is_none() && s_i >= 0 {
                if &s[s_i as usize..(s_i + 1) as usize] == "#" {
                    s_back += 1;
                } else if s_back > 0 {
                    s_back -= 1;
                } else {
                    s_char = Some(&s[s_i as usize..(s_i + 1) as usize]);
                }
                s_i -= 1;
            }

            while t_char.is_none() && t_i >= 0 {
                if &t[t_i as usize..(t_i + 1) as usize] == "#" {
                    t_back += 1;
                } else if t_back > 0 {
                    t_back -= 1;
                } else {
                    t_char = Some(&t[t_i as usize..(t_i + 1) as usize]);
                }
                t_i -= 1;
            }

            if s_char != t_char {
                return false;
            }
        }

        true
    }
}
