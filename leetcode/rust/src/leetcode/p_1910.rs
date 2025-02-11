// 1910. Remove All Occurrences of a Substring
// -------------------------------------------

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut letters = vec![];
        let n = part.len();
        let search: Vec<char> = part.chars().collect();

        s.chars().for_each(|c| {
            letters.push(c);
            while letters.len() >= n {
                let m = letters.len();
                if &letters[(m - n)..] == search {
                    letters.truncate(m - n);
                } else {
                    break;
                }
            }
        });
        letters.iter().collect::<String>()
    }
}
