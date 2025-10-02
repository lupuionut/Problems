// 3100. Water Bottles II
// ----------------------

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut ans = 0;
        let mut empty = 0;

        while num_bottles > 0 || empty >= num_exchange {
            while empty >= num_exchange {
                num_bottles += 1;
                empty -= num_exchange;
                num_exchange += 1;
            }
            ans += num_bottles;
            empty += num_bottles;
            num_bottles = 0;
        }

        ans
    }
}
