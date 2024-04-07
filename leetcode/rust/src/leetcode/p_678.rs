// 678. Valid Parenthesis String
// -----------------------------

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        fn validate(
            s: &Vec<char>,
            i: usize,
            size: i32,
            cache: &mut Vec<Vec<Option<bool>>>,
        ) -> bool {
            if i == s.len() {
                if size == 0 {
                    return true;
                }
                return false;
            }

            if cache[i][size as usize] != None {
                return cache[i][size as usize].unwrap();
            }

            let mut ans = false;

            if s[i] == '(' {
                ans = validate(s, i + 1, size + 1, cache);
            } else if s[i] == ')' {
                if size - 1 >= 0 {
                    ans = validate(s, i + 1, size - 1, cache);
                } else {
                    ans = false;
                }
            } else {
                ans = (validate(s, i + 1, size + 1, cache)
                    || (size - 1 >= 0 && validate(s, i + 1, size - 1, cache))
                    || validate(s, i + 1, size, cache));
            }

            cache[i][size as usize] = Some(ans);
            ans
        }

        let mut cache = vec![vec![None; s.len()]; s.len()];
        let s = s.chars().collect::<Vec<_>>();
        validate(&s, 0, 0, &mut cache)
    }
}
