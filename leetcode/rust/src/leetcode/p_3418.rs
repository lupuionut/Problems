// 3418. Maximum Amount of Money Robot Can Earn
// --------------------------------------------
impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        fn visit(
            i: usize,
            j: usize,
            coins: &Vec<Vec<i32>>,
            neutralize: i32,
            cache: &mut Vec<Vec<Vec<Option<i32>>>>,
        ) -> i32 {
            if i == coins.len() - 1 && j == coins[0].len() - 1 {
                if coins[i][j] < 0 && neutralize > 0 {
                    return 0;
                } else {
                    return coins[i][j];
                }
            }
            if let Some(v) = cache[i][j][neutralize as usize] {
                return v;
            }
            let mut best = i32::MIN;
            if coins[i][j] < 0 && neutralize > 0 {
                if j < coins[0].len() - 1 {
                    best = best.max(visit(i, j + 1, coins, neutralize - 1, cache));
                }
                if i < coins.len() - 1 {
                    best = best.max(visit(i + 1, j, coins, neutralize - 1, cache));
                }
            }
            if j < coins[0].len() - 1 {
                best = best.max(coins[i][j] + visit(i, j + 1, coins, neutralize, cache));
            }
            if i < coins.len() - 1 {
                best = best.max(coins[i][j] + visit(i + 1, j, coins, neutralize, cache));
            }
            cache[i][j][neutralize as usize] = Some(best);
            best
        }
        let mut cache = vec![vec![vec![None; 3]; coins[0].len()]; coins.len()];
        visit(0, 0, &coins, 2, &mut cache)
    }
}
