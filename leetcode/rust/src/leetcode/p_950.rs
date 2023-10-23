// 950. Reveal Cards In Increasing Order
// -------------------------------------

use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        let mut ans = deck.clone();

        deck.sort_by(|a, b| b.cmp(&a));

        let mut deque: VecDeque<i32> = VecDeque::new();
        deck.iter().for_each(|n| {
            if deque.len() > 1 {
                let new_front = deque.pop_back().unwrap();
                deque.push_front(new_front);
            };
            deque.push_front(*n);
        });

        deque.iter().enumerate().for_each(|(i, &v)| {
            ans[i] = v;
        });

        ans
    }
}
