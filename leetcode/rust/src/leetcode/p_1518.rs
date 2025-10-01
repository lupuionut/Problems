// 1518. Water Bottles
// -------------------

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut empty = num_bottles;
        while empty >= num_exchange {
            let drink = empty / num_exchange;
            ans += drink;
            empty = drink + (empty - (num_exchange * drink));
        }
        ans
    }
}
