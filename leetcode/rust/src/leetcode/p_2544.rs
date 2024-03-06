// 2544. Alternating Digit Sum
// ---------------------------

impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut digits = vec![];
        while n > 0 {
            let digit = n % 10;
            digits.push(digit);
            n /= 10;
        }

        let mut sign = if digits.len() % 2 == 0 { -1 } else { 1 };
        let mut acc = 0;
        digits.iter().for_each(|&d| {
            acc += (d * sign);
            sign = -sign;
        });

        acc
    }
}
