// 2042. Check if Numbers Are Ascending in a Sentence
// --------------------------------------------------

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut curr = -1;
        let mut ans = true;
        s.split(" ")
            .map(|p| p.parse::<i32>())
            .filter(|s| s.is_ok())
            .map(|e| e.unwrap())
            .for_each(|e| {
                if e <= curr {
                    ans = false;
                } else {
                    curr = e;
                }
            });
        ans
    }
}
