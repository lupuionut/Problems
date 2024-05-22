// 131. Palindrome Partitioning
// ----------------------------

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn generate(i: usize, curr: &mut Vec<String>, s: &Vec<char>, ans: &mut Vec<Vec<String>>) {
            if i == s.len() {
                ans.push(curr.clone());
                return;
            }

            for e in i..s.len() {
                let s1 = &s[i..e + 1].to_vec().iter().collect::<String>();
                let s2 = &s[i..e + 1].to_vec().iter().rev().collect::<String>();
                if s1 == s2 {
                    curr.push(s1.to_string());
                    generate(e + 1, curr, s, ans);
                    curr.pop();
                }
            }
        }

        let mut ans = vec![];
        let mut curr = vec![];
        let s = s.chars().collect::<Vec<_>>();
        generate(0, &mut curr, &s, &mut ans);
        ans
    }
}
