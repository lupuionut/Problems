impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; 31];
        let mut i = 2;
        cache[1] = 1;
        while i <= n {
            cache[i] = cache[i - 1] + cache[i - 2];
            i += 1;
        }
        cache[n]
    }
}
