// 1952. Three Divisors

impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut counter = 2;
        for i in 2..(n / 2 + 1) {
            if n % i == 0 {
                counter += 1
            }
            if counter > 3 {
                return false;
            }
        }
        if counter != 3 {
            return false;
        }
        true
    }
}
