// 1869. Longer Contiguous Segments of Ones than Zeros
// ---------------------------------------------------

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max0 = 0;
        let mut max1 = 0;
        let mut temp = 0;

        for b in s.as_bytes() {
            if *b == b'0' {
                temp += 1;
                max0 = max0.max(temp);
            } else {
                temp = 0;
            }
        }

        temp = 0;

        for b in s.as_bytes() {
            if *b == b'1' {
                temp += 1;
                max1 = max1.max(temp);
            } else {
                temp = 0;
            }
        }

        if max1 > max0 {
            true
        } else {
            false
        }
    }
}
