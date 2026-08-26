// 2904. Shortest and Lexicographically Smallest Beautiful String
// --------------------------------------------------------------
impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let n = s.len();
        let mut ans = "";
        for i in 0..n {
            for j in i+1..=n {
                let t = &s[i..j];
                let mut count = 0;
                for c in t.chars() {
                    if c == '1' {
                        count += 1;
                    }
                }
                if count == k {
                    if t.len() < ans.len() {
                        ans = t;
                    } else if ans.len() == t.len() {
                        if t < ans {
                            ans = t;
                        }   
                    } else if ans.len() == 0 {
                        ans = t;
                    }
                }
            }
        }

        ans.to_string()
    }
}
