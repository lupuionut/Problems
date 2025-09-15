// 1935. Maximum Number of Words You Can Type
// ------------------------------------------

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut can_type = true;
        let mut count = 0;
        let mut broken = [false; 26];
        broken_letters.chars().for_each(|c| {
            let k = c as usize - 97;
            broken[k] = true;
        });

        text.chars().for_each(|c| {
            if c == ' ' {
                if can_type == true {
                    count += 1;
                }
                can_type = true;
            } else {
                let k = c as usize - 97;
                if broken[k] == true {
                    can_type = false;
                }
            }
        });
        if can_type {
            count += 1;
        }
        count
    }
}
