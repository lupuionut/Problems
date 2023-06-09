// 1790. Check if One String Swap Can Make Strings Equal
// -----------------------------------------------------

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut different: Vec<&u8> = Vec::new();
        let mut s1 = s1.as_bytes().iter();
        let mut s2 = s2.as_bytes().iter();

        while let Some(c1) = s1.next() {
            if let Some(c2) = s2.next() {
                if c1 != c2 {
                    different.push(c1);
                    different.push(c2);
                }
            }
        }

        if different.len() == 0 {
            return true;
        } else if different.len() == 4 {
            if different[0] == different[3] && different[1] == different[2] {
                return true;
            }
        }
        false
    }
}
