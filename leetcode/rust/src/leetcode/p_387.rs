// 387. First Unique Character in a String
// ---------------------------------------

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letters = vec![(None, 0); 26];
        s.chars().enumerate().for_each(|(k, v)| {
            let key = v as usize - 97;
            let (position, occurences) = letters[key];
            if occurences == 0 {
                letters[key] = (Some(k), 1);
            } else {
                letters[key] = (position, occurences + 1);
            }
        });
        let ans = letters
            .iter()
            .filter(|(_, v)| *v == 1)
            .map(|(p, _)| p.unwrap())
            .min();
        if let Some(ans) = ans {
            return ans as i32;
        }
        -1
    }
}
