// 2109. Adding Spaces to a String
// -------------------------------

impl Solution {
    pub fn add_spaces(s: String, mut spaces: Vec<i32>) -> String {
        spaces.sort();
        let mut chars = vec![];
        let mut ptr = 0;
        s.chars().enumerate().for_each(|(k, v)| {
            if ptr < spaces.len() && k as i32 == spaces[ptr] {
                chars.push(' ');
                ptr += 1;
            }
            chars.push(v);
        });

        chars.iter().collect::<String>()
    }
}
