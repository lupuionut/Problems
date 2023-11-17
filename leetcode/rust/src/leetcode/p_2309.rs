// 2309. Greatest English Letter in Upper and Lower Case
// -----------------------------------------------------

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut chars_occurance = vec![0; 26];

        s.chars().for_each(|c| {
            if c.is_lowercase() {
                let key = (c as i32 - 97) as usize;
                chars_occurance[key] = chars_occurance[key] | 1;
            } else {
                let key = (c as i32 - 65) as usize;
                chars_occurance[key] = chars_occurance[key] | (1 << 1);
            }
        });

        let mut highest: i32 = -1;
        chars_occurance.iter().enumerate().for_each(|(k, &v)| {
            if v == 3 {
                highest = k as i32;
            }
        });

        if highest == -1 {
            String::new()
        } else {
            char::from_u32((65 + highest) as u32).unwrap().to_string()
        }
    }
}
