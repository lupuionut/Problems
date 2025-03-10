// 3306. Count of Substrings Containing Every Vowel and K Consonants II
// --------------------------------------------------------------------

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let chars = word.chars().collect::<Vec<char>>();

        fn count(chars: &Vec<char>, k: i32) -> i64 {
            let mut ans = 0;
            let mut l = 0;
            let mut vowels = [0; 5];
            let mut cons = 0;
            for r in 0..chars.len() {
                match chars[r] {
                    'a' => vowels[0] += 1,
                    'e' => vowels[1] += 1,
                    'i' => vowels[2] += 1,
                    'o' => vowels[3] += 1,
                    'u' => vowels[4] += 1,
                    _ => cons += 1,
                };
                while cons >= k && vowels.iter().all(|&x| x > 0) {
                    ans += (chars.len() - r) as i64;
                    match chars[l] {
                        'a' => vowels[0] -= 1,
                        'e' => vowels[1] -= 1,
                        'i' => vowels[2] -= 1,
                        'o' => vowels[3] -= 1,
                        'u' => vowels[4] -= 1,
                        _ => cons -= 1,
                    };
                    l += 1;
                }
            }
            ans
        }

        count(&chars, k) - count(&chars, k + 1)
    }
}
