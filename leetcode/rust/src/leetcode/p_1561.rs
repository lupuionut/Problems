// 1561. Maximum Number of Coins You Can Get
// -----------------------------------------

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let mut ans = 0;
        let end = piles.len() * 2 / 3;
        piles.sort_by(|a, b| b.cmp(a));

        for i in 0..end {
            if i & 1 == 1 {
                ans += piles[i];
            }
        }

        ans
    }
}
