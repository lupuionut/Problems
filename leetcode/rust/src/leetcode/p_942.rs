// 942. DI String Match
// --------------------

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut I = 0;
        let mut D = s.len() as i32;
        let mut ans = vec![];

        s.as_bytes().iter().for_each(|&c| {
            if c == b'I' {
                ans.push(I);
                I += 1;
            } else {
                ans.push(D);
                D -= 1;
            }
        });

        ans.push(I);
        ans
    }
}
