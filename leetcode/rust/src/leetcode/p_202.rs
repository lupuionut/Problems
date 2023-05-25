// 202. Happy Number
// -----------------

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut ans = false;
        let mut hs: HashSet<i32> = HashSet::new();

        fn square(n: i32) -> i32 {
            let mut n = n;
            let mut res = 0;
            while n != 0 {
                let d = n % 10;
                res += (d * d);
                n /= 10;
            }
            res
        }

        loop {
            hs.insert(n);
            n = square(n);

            if n == 1 {
                ans = true;
                break;
            }
            if hs.contains(&n) {
                break;
            }
        }

        ans
    }
}
