// 518. Coin Change II
// -------------------

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        let mut cache: Vec<Vec<i32>> = vec![vec![-1; (amount + 1) as usize]; coins.len()];
        coins.sort();
        Solution::dp(amount, 0, &coins, &mut cache)
    }

    pub fn dp(target: i32, i: usize, coins: &Vec<i32>, cache: &mut Vec<Vec<i32>>) -> i32 {
        if target < 0 {
            return 0;
        }
        if target == 0 {
            return 1;
        }
        if i == coins.len() {
            return 0;
        }
        if cache[i][target as usize] != -1 {
            return cache[i][target as usize];
        }
        cache[i][target as usize] = Solution::dp(target - coins[i], i, coins, cache)
            + Solution::dp(target, i + 1, coins, cache);

        cache[i][target as usize]
    }
}
