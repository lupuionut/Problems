// 2523. Closest Prime Numbers in Range
// ------------------------------------

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = vec![true; (right + 1) as usize];
        let mut ans = vec![];
        primes[0] = false;
        primes[1] = false;
        let mut p = 2;
        while p * p <= right {
            if primes[p as usize] == true {
                let mut i = p * p;
                while i <= right {
                    primes[i as usize] = false;
                    i += p;
                }
            }
            p += 1;
        }

        let mut last = -1;
        for i in left..=right {
            if primes[i as usize] == true {
                if ans.len() == 0 || ans.len() == 1 {
                    ans.push(i);
                } else {
                    let delta = ans[1] - ans[0];
                    let possible = i - last;
                    if possible < delta {
                        ans[0] = last;
                        ans[1] = i;
                    }
                }
                last = i;
            }
        }

        if ans.len() != 2 {
            return vec![-1, -1];
        }
        ans
    }
}
