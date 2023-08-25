// 97. Interleaving String
// -----------------------

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut cache = vec![vec![-1; s2.len() + 1]; s1.len() + 1];

        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        fn check(
            i1: usize,
            i2: usize,
            i3: usize,
            s1: &[u8],
            s2: &[u8],
            s3: &[u8],
            cache: &mut Vec<Vec<i32>>,
        ) -> bool {
            if i3 == s3.len() {
                return true;
            }

            if cache[i1][i2] != -1 {
                if cache[i1][i2] == 1 {
                    return true;
                }
                return false;
            }

            if i1 < s1.len()
                && s3[i3] == s1[i1]
                && check(i1 + 1, i2, i3 + 1, s1, s2, s3, cache) == true
            {
                cache[i1][i2] = 1;
                return true;
            }
            if i2 < s2.len()
                && s3[i3] == s2[i2]
                && check(i1, i2 + 1, i3 + 1, s1, s2, s3, cache) == true
            {
                cache[i1][i2] = 1;
                return true;
            }
            cache[i1][i2] = 0;
            return false;
        }

        check(0, 0, 0, s1, s2, s3, &mut cache)
    }
}
