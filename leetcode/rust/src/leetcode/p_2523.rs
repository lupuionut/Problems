// 2523. Closest Prime Numbers in Range
// ------------------------------------
// Sieve of Eratosthenes, primes

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut previous = -1_000_000;
        let mut delta = 1_000_000;
        let mut i = left;
        let mut ans = vec![-1; 2];

        fn sieve(from: usize, to: usize) -> Vec<bool> {
            let mut ans = vec![true; to + 1];
            let mut n = 1;

            while n <= to {
                if n == 1 {
                    ans[n] = false;
                    n += 1;
                    continue;
                }
                if ans[n] == false {
                    n += 1;
                    continue;
                }
                let mut m = 2;
                let mut p = n;
                while p * m <= to {
                    ans[(p * m)] = false;
                    m += 1;
                }
                n += 1;
            }
            ans[from..=to].to_vec()
        }

        let primes = sieve(i as usize, right as usize);

        while i <= right {
            let k = (i - left) as usize;
            if primes[k] == true {
                if (i - previous) < delta {
                    ans[0] = previous;
                    ans[1] = i;
                    delta = i - previous;
                }
                previous = i;
            }
            i += 1;
        }

        ans
    }
}
