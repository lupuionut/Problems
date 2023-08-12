// 2129. Capitalize the Title
// --------------------------

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut ans: Vec<char> = vec![];
        let mut tmp: Vec<char> = vec![];

        title.chars().for_each(|c| {
            if c == ' ' {
                if tmp.len() > 0 {
                    if tmp.len() > 2 {
                        tmp[0] = tmp[0].to_ascii_uppercase();
                    }
                    tmp.push(' ');
                    ans.append(&mut tmp);
                    tmp.clear();
                }
            } else {
                tmp.push(c.to_ascii_lowercase())
            }
        });

        if tmp.len() > 2 {
            tmp[0] = tmp[0].to_ascii_uppercase();
        }
        ans.append(&mut tmp);
        ans.iter().collect::<String>()
    }
}
