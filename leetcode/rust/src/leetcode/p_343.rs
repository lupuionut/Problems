// 343. Integer Break
// ------------------

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut cache = vec![-1; (n + 1) as usize];
        fn dp(n: i32, broken: bool, cache: &mut Vec<i32>) -> i32 {
            if n == 1 {
                return 1;
            }
            if cache[n as usize] != -1 {
                return cache[n as usize];
            }
            let mut best = 0;
            if broken == true {
                best = n;
            }
            for i in 1..n {
                best = best.max(i * dp(n - i, true, cache));
            }
            cache[n as usize] = best;
            best
        }
        dp(n, false, &mut cache)
    }
}
