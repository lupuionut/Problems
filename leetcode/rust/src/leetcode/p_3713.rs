// 3713. Longest Balanced Substring I
// ----------------------------------
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut ans = i32::MIN;
        let s: Vec<char> = s.chars().collect();

        for l in 0..s.len() {
            let mut freq = vec![0; 26];
            for r in l..s.len() {
                let k = s[r] as usize - 97;
                freq[k] += 1;
                let mut b = true;
                let max = *freq.iter().max().unwrap();
                for i in 0..26 {
                    if freq[i] != max && freq[i] != 0 {
                        b = false;
                        break;
                    }
                }
                if b == true {
                    ans = ans.max((r - l + 1) as i32);
                }
            }
        }

        ans
    }
}
