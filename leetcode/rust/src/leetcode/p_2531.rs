// 2531. Make Number of Distinct Characters Equal
// ----------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut ha = HashMap::new();
        let mut hb = HashMap::new();

        for c in word1.chars() {
            ha.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        for c in word2.chars() {
            hb.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        for &a in ha.keys() {
            for &b in hb.keys() {
                let mut ha_clone = ha.clone();
                let mut hb_clone = hb.clone();

                ha_clone.entry(a).and_modify(|v| *v -= 1);
                ha_clone.entry(b).and_modify(|v| *v += 1).or_insert(1);
                hb_clone.entry(b).and_modify(|v| *v -= 1);
                hb_clone.entry(a).and_modify(|v| *v += 1).or_insert(1);
                if let Some(v) = ha_clone.get(&a) {
                    if *v == 0 {
                        ha_clone.remove(&a);
                    }
                }
                if let Some(v) = hb_clone.get(&b) {
                    if *v == 0 {
                        hb_clone.remove(&b);
                    }
                }
                if ha_clone.len() == hb_clone.len() {
                    return true;
                }
            }
        }

        false
    }
}
