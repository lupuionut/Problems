// 1247. Minimum Swaps to Make Strings Equal
// -----------------------------------------
//  x x    y y
//  y y    x x  --> 1 swap

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut xy = 0;
        let mut yx = 0;
        let mut iter = s1.as_bytes().iter().zip(s2.as_bytes().iter());

        while let Some((&t, &b)) = iter.next() {
            if t != b {
                if t == b'x' {
                    xy += 1;
                } else {
                    yx += 1;
                }
            }
        }

        if (xy + yx) % 2 == 1 {
            return -1;
        }

        xy / 2 + yx / 2 + xy % 2 + yx % 2
    }
}
