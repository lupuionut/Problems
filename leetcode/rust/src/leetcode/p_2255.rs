// 2255. Count Prefixes of a Given String

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut ans = 0;
        for word in words {
            let length = word.len();
            if length > s.len() {
                continue;
            }
            let possible = word.get(0..length).unwrap();
            let substring = s.get(0..length).unwrap();
            if possible == substring {
                ans += 1;
            }
        }
        ans
    }
}
