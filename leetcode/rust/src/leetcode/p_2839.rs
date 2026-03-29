// 2839. Check if Strings Can be Made Equal With Operations I
// ----------------------------------------------------------
impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1_0 = [0; 26];
        let mut s1_1 = [0; 26];
        let mut s2_0 = [0; 26];
        let mut s2_1 = [0; 26];
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();

        if s1.len() != s2.len() {
            return false;
        }
        let n = s1.len();
        for i in 0..n {
            let i = i % n;
            if i % 2 == 0 {
                let mut k = s1[i] as usize - 97;
                s1_0[k] += 1;
                k = s2[i] as usize - 97;
                s2_0[k] += 1;
            } else {
                let mut k = s1[i] as usize - 97;
                s1_1[k] += 1;
                k = s2[i] as usize - 97;
                s2_1[k] += 1;
            }
        }

        s1_0 == s2_0 && s1_1 == s2_1
    }
}
