// 3403. Find the Lexicographically Largest String From the Box I
// --------------------------------------------------------------

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let mut best = String::new();
        let n = word.len();
        let len = n - (num_friends - 1) as usize;
        for i in 0..n {
            let mx = n.min(i + len);
            if best < word[i..mx].to_string() {
                best = word[i..mx].to_string();
            }
        }
        best
    }
}
