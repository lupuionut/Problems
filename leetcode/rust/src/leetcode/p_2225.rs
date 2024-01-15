// 2225. Find Players With Zero or One Losses
// ------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = HashSet::new();
        let mut losers = HashSet::new();
        let mut ignore = HashSet::new();

        matches.iter().for_each(|m| {
            let winner = m[0];
            let loser = m[1];

            if !ignore.contains(&winner) && !losers.contains(&winner) {
                winners.insert(winner);
            }

            if !ignore.contains(&loser) {
                if winners.contains(&loser) {
                    winners.remove(&loser);
                    losers.insert(loser);
                } else {
                    if losers.contains(&loser) {
                        losers.remove(&loser);
                        ignore.insert(loser);
                    } else {
                        losers.insert(loser);
                    }
                }
            }
        });

        let mut winners = winners.into_iter().collect::<Vec<_>>();
        let mut losers = losers.into_iter().collect::<Vec<_>>();
        winners.sort();
        losers.sort();

        vec![winners, losers]
    }
}
