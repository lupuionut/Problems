// 1358. Number of Substrings Containing All Three Characters
// ----------------------------------------------------------

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ans = 0;
        let mut count = [0; 3];
        let chars = s.chars().collect::<Vec<char>>();
        let mut l = 0;

        for r in 0..s.len() {
            match chars[r] {
                'a' => count[0] += 1,
                'b' => count[1] += 1,
                'c' => count[2] += 1,
                _ => {}
            }
            while count.iter().all(|&x| x >= 1) {
                ans += (s.len() - r) as i32;
                match chars[l] {
                    'a' => count[0] -= 1,
                    'b' => count[1] -= 1,
                    'c' => count[2] -= 1,
                    _ => {}
                }
                l += 1;
            }
        }

        ans
    }
}
