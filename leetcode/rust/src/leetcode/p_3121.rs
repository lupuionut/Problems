// 3121. Count the Number of Special Characters II
// -----------------------------------------------
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut f = vec![(0,0); 26];
        word.chars().for_each(|c| {
            let mut k = c as usize;
            // lowercase
            if k >= 97 {
                k -= 97;
                if f[k].1 == 0 {
                    f[k].0 = 1;
                } else {
                    f[k].0 = 0;
                }
            // uppercase
            } else {
                k -= 65;
                f[k].1 = 1;
            }
        });
        let mut ans = 0;
        for i in 0..26 {
            if f[i].0 == 1 && f[i].0 == f[i].1 {
                ans += 1;
            }
        }
        ans
    }
}
