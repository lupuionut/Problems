// 1318. Minimum Flips to Make a OR b Equal to c
// ---------------------------------------------

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut flips = 0;
        let mut a = a;
        let mut b = b;
        let mut c = c;

        while a > 0 || b > 0 || c > 0 {
            // if last bits a OR b != last bit c
            if (a & 1) | (b & 1) != (c & 1) {
                // if last bit from a and b are the same
                if (a & 1) ^ (b & 1) == 0 {
                    // last bit form c is 0 -> both a and b have 1 as last bit
                    if c & 1 == 0 {
                        flips += 2;
                    // last bit from c is 1 -> both a and b have 0 as last bit
                    } else {
                        flips += 1;
                    }
                // if last bit from a and b differs, we need only one flip
                } else {
                    flips += 1;
                }
            }
            a = a >> 1;
            b = b >> 1;
            c = c >> 1;
        }

        flips
    }
}
