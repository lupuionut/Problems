// 846. Hand of Straights
// ----------------------

use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut cards: HashMap<i32, i32> = HashMap::new();
        let mut ans = true;
        hand.into_iter().for_each(|n| {
            cards.entry(n).and_modify(|c| *c += 1).or_insert(1);
        });
        let mut keys = cards.keys().map(|&n| n).collect::<Vec<i32>>();
        keys.sort();

        for key in &keys {
            while let Some(&val) = cards.get(&key) {
                if val == 0 {
                    break;
                } else {
                    for next in key + 1..key + group_size {
                        let n = cards.get(&next);
                        if let Some(&v) = n {
                            if v == 0 {
                                return false;
                            }
                        } else {
                            return false;
                        }

                        cards.entry(next).and_modify(|c| *c -= 1);
                    }
                }
                cards.entry(*key).and_modify(|c| *c -= 1);
            }
        }
        ans
    }
}
