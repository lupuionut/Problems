// 3163. String Compression III
// ----------------------------

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut curr = (None, 0);
        let mut ans = vec![];

        word.chars().for_each(|c| {
            if let Some(chr) = curr.0 {
                if chr == c {
                    curr.1 += 1;
                    if curr.1 == 9 {
                        ans.push('9');
                        ans.push(c);
                        curr = (None, 0);
                    }
                } else {
                    ans.push(char::from_digit(curr.1, 10).unwrap());
                    ans.push(chr);
                    curr.0 = Some(c);
                    curr.1 = 1;
                }
            } else {
                curr.0 = Some(c);
                curr.1 = 1;
            }
        });

        if let Some(c) = curr.0 {
            ans.push(char::from_digit(curr.1, 10).unwrap());
            ans.push(c);
        }

        ans.iter().collect::<String>()
    }
}
