// 3227. Vowels Game in a String
// -----------------------------
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        if s.contains(&['a', 'e', 'i', 'o', 'u']) {
            return true;
        }
        false
    }
}
