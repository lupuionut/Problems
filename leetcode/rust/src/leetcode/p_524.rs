// 524. Longest Word in Dictionary through Deleting
// ------------------------------------------------

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut ans: Option<String> = None;

        fn possible_string(s1: &str, s2: &str) -> Option<String> {
            let mut chars_s1 = s1.chars();
            let mut chars_s2 = s2.chars();
            let mut chars = vec![];

            let mut c1 = chars_s1.next();
            let mut c2 = chars_s2.next();

            loop {
                if c1.is_none() || c2.is_none() {
                    break;
                }

                let a = c1.unwrap();
                let b = c2.unwrap();

                if a == b {
                    chars.push(a);
                    c1 = chars_s1.next();
                    c2 = chars_s2.next();
                } else {
                    c1 = chars_s1.next();
                }
            }

            if chars.len() == s2.len() {
                return Some(chars.iter().collect::<String>());
            }
            None
        }

        let s1 = s.as_str();
        dictionary.iter().for_each(|word| {
            let s2 = word.as_str();
            match possible_string(s1, s2) {
                Some(val) => {
                    if ans.is_none() {
                        ans = Some(val);
                    } else {
                        let c = ans.clone().unwrap();
                        if c.len() == val.len() {
                            if c > val {
                                ans = Some(val);
                            }
                        } else if val.len() > c.len() {
                            ans = Some(val);
                        }
                    }
                }
                None => {}
            }
        });

        match ans {
            Some(val) => val,
            None => "".to_string(),
        }
    }
}
