// 2405. Optimal Partition of String
// ---------------------------------

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut ans = 1;
        let mut chars = vec![false; 26];

        s.as_bytes().iter().for_each(|&c| {
            let key = c as usize - 97;
            if chars[key] == true {
                ans += 1;
                chars = vec![false; 26];
            }
            chars[key] = true;
        });
        ans
    }
}
