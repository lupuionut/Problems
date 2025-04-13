// 1922. Count Good Numbers
// ------------------------

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let mods = 1_000_000_007i64;
        let mut odd = n / 2;
        let mut even = n / 2;
        if n % 2 == 1 {
            even += 1;
        }

        fn power(mut base: i64, mut exponent: i64, mods: i64) -> i64 {
            let mut ans = 1;
            while exponent > 0 {
                if exponent % 2 == 1 {
                    ans = ans * base % mods;
                }
                base = (base * base) % mods;
                exponent >>= 1;
            }
            ans % mods
        }

        (power(5, even, mods) % mods * power(4, odd, mods) % mods) as i32
    }
}
