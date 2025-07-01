// 3330. Find the Original Typed String I
// --------------------------------------

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ans = 1;
        let mut curr = 0u8;

        for &b in word.as_bytes() {
            if b != curr {
                curr = b;
            } else {
                ans += 1;
            }
        }
        ans
    }
}
