// 1140. Stone Game II
// -------------------

use core::i32::MIN;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![-1; piles.len() + 1]; piles.len() + 1];

        fn _min(a: i32, b: i32) -> i32 {
            if a < b {
                return a;
            }
            b
        }

        fn _max(a: i32, b: i32) -> i32 {
            if a > b {
                return a;
            }
            b
        }

        fn get_delta(idx: usize, m: usize, cache: &mut Vec<Vec<i32>>, piles: &Vec<i32>) -> i32 {
            if idx == piles.len() {
                return 0;
            }

            if cache[idx][m] != -1 {
                return cache[idx][m];
            }

            let mut best: i32 = MIN;
            for x in 1..=(2 * m) {
                let end = if (idx + x) < piles.len() {
                    idx + x
                } else {
                    piles.len()
                };
                let my_stones: i32 = piles[idx..end].iter().sum();
                let op_stones = get_delta(
                    _min(end as i32, piles.len() as i32) as usize,
                    _min(_max(m as i32, x as i32), piles.len() as i32) as usize,
                    cache,
                    piles,
                );
                best = _max(best, my_stones - op_stones);
            }
            cache[idx][m] = best;
            best
        }

        let delta = get_delta(0, 1, &mut cache, &piles);
        let total: i32 = piles.iter().sum();

        return (total + delta) / 2;
    }
}
