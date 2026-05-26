// 3120. Count the Number of Special Characters I
// ----------------------------------------------
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut letter = vec![(0,0); 26];
        word.chars().for_each(|c| {
            let mut k = c as usize;
            if k >= 97 {
                k -= 97;
                letter[k].0 = 1;
            } else {
                k -= 65;
                letter[k].1 = 1;
            }
        });
        let mut ans = 0;
        for i in 0..26 {
            if letter[i].0 == 1 && letter[i].0 == letter[i].1 {
                ans += 1;
            }
        }
        ans
    }
}
