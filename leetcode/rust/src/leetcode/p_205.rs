// 205. Isomorphic Strings
// -----------------------

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut it = s.chars().zip(t.chars());
        let mut from_a_b = HashMap::new();
        let mut from_b_a = HashMap::new();

        for (a, b) in it {
            if let Some(&ab) = from_a_b.get(&a) {
                if ab != b {
                    return false;
                }
            } else {
                if let Some(&ba) = from_b_a.get(&b) {
                    if ba != a {
                        return false;
                    }
                }
                from_a_b.insert(a, b);
                from_b_a.insert(b, a);
            }
        }

        true
    }
}
