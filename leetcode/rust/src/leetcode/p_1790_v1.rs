// 1790. Check if One String Swap Can Make Strings Equal
// -----------------------------------------------------

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        let mut change = None;
        let mut used = false;
        let mut iter = s1.chars().zip(s2.chars());

        while let Some((a, b)) = iter.next() {
            if a != b {
                if used == true && !change.is_none() {
                    return false;
                }
                if let Some((ca, cb)) = change {
                    if ca != b || cb != a {
                        return false;
                    }
                    used = true;
                } else {
                    change = Some((a, b));
                }
            }
        }

        if change.is_some() && used == false {
            return false;
        }

        true
    }
}
