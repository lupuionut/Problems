// 1897. Redistribute Characters to Make All Strings Equal
// -------------------------------------------------------

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut letters = vec![0; 26];
        let mut len = words.len() as i32;

        words.iter().map(|s| s.chars()).for_each(|chars| {
            chars.for_each(|c| {
                let key = (c as usize) - 97;
                letters[key] += 1;
            });
        });

        for value in letters {
            if value != 0 && value % len != 0 {
                return false;
            }
        }

        true
    }
}
