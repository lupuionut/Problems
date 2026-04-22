// 2452. Words Within Two Edits of Dictionary
// ------------------------------------------
impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        fn is_good(q: &String, d: &String) -> bool {
            let iter = q.chars().zip(d.chars());
            let mut changes = 0;
            for (a, b) in iter {
                if a != b {
                    changes += 1;
                }
            }
            changes <= 2
        }
        for i in 0..queries.len() {
            for j in 0..dictionary.len() {
                if is_good(&queries[i], &dictionary[j]) {
                    ans.push(queries[i].clone());
                    break;
                }
            }
        }
        ans
    }
}
