// 1155. Number of Dice Rolls With Target Sum
// ------------------------------------------

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        fn count_rolls(m: i32, n: i32, k: i32, target: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
            // we finished the number of dices (n)
            // if we remained with 0 in target it's a way to roll
            if m > n {
                if target == 0 {
                    return 1;
                }
                return 0;
            }

            if cache[m as usize][target as usize] != -1 {
                return cache[m as usize][target as usize];
            }

            let mut ans = 0;
            let max = k.min(target);
            for i in 1..=max {
                ans += (count_rolls(m + 1, n, k, target - i, cache) % 1_000_000_007);
                ans %= 1_000_000_007;
            }
            cache[m as usize][target as usize] = ans % 1_000_000_007;
            cache[m as usize][target as usize]
        }

        let mut cache = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        count_rolls(1, n, k, target, &mut cache)
    }
}
