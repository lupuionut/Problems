// 204. Count Primes
// -----------------
// Sieve of Eratosthenes

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut ans = 0;
        let mut i = 2;
        let mut notprime: Vec<i8> = vec![0; n as usize];

        while i * i < n {
            if notprime[i as usize] == 0 {
                let mut j = i * i;
                while j < n {
                    notprime[j as usize] = 1;
                    j += i;
                }
            }
            i += 1;
        }

        notprime.iter().filter(|&x| *x == 0).count() as i32 - 2
    }
}
