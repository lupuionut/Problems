// 1317. Convert Integer to the Sum of Two No-Zero Integers
// --------------------------------------------------------

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn contains_zero(mut n: i32) -> bool {
            if n == 0 {
                return true;
            }
            while n > 0 {
                if n % 10 == 0 {
                    return true;
                }
                n /= 10;
            }
            false
        }

        for i in 1..n {
            if !contains_zero(i) && !contains_zero(n - i) {
                return vec![i, n - i];
            }
        }
        vec![]
    }
}
