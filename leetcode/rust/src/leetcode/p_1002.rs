// 1002. Find Common Characters
// ----------------------------

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut letters = vec![vec![0; 26]; n];
        let mut ans = vec![];

        words.iter().enumerate().for_each(|(i, &ref word)| {
            word.chars().for_each(|c| {
                let key = c as usize - 97;
                letters[i][key] += 1;
            });
        });

        for i in 0..26 {
            let mut count = i32::MAX;
            for j in 0..n {
                count = count.min(letters[j][i]);
            }
            if count > 0 {
                let letter = char::from_u32(i as u32 + 97).unwrap().to_string();
                let mut temp = vec![letter; count as usize];
                ans.append(&mut temp);
            }
        }
        ans
    }
}
