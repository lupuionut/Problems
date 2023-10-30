// 2048. Next Greater Numerically Balanced Number
// ----------------------------------------------

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        fn is_balanced(mut n: i32) -> bool {
            let mut digits = vec![0; 10];
            let mut ans = true;
            while n > 0 {
                let digit = n % 10;
                digits[digit as usize] += 1;
                n /= 10;
            }
            digits.iter().enumerate().for_each(|(k, &v)| {
                if v != 0 && k as i32 != v {
                    ans = false;
                }
            });
            ans
        }

        let mut n = n + 1;
        loop {
            if is_balanced(n) {
                return n;
            }
            n += 1;
        }
    }
}
