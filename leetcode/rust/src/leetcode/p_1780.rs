// 1780. Check if Number is a Sum of Powers of Three
// -------------------------------------------------

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        fn dp(p: u32, n: i32) -> bool {
            let m = 3_i32.pow(p);
            if n == 0 {
                return true;
            }
            if m > n || n < 0 {
                return false;
            }

            return dp(p + 1, n - m) || dp(p + 1, n);
        }
        dp(0, n)
    }
}
