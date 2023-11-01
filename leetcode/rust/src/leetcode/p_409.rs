// 409. Longest Palindrome
// -----------------------

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq = vec![0; 52];
        let mut ans = 0;

        for b in s.as_bytes() {
            let key = if b.is_ascii_lowercase() {
                b - 97
            } else {
                b - 65 + 26
            };
            freq[key as usize] += 1;
        }

        let mut center = false;
        freq.iter().for_each(|n| {
            if n % 2 == 0 {
                ans += n;
            } else {
                if center == true {
                    ans += (n - 1);
                } else {
                    ans += n;
                    center = true;
                }
            }
        });

        ans
    }
}
