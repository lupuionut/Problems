// 91. Decode Ways
// ---------------

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn decode(i: usize, s: &str, cache: &mut Vec<i32>) -> i32 {
            if i >= s.len() {
                return 1;
            }

            if cache[i] != -1 {
                return cache[i];
            }

            let mut ans = 0;
            let first = s[i..i + 1].parse::<i32>().unwrap();
            if first != 0 {
                ans += decode(i + 1, s, cache);
                if i + 2 <= s.len() {
                    let second = s[i..i + 2].parse::<i32>().unwrap();
                    if second <= 26 {
                        ans += decode(i + 2, s, cache);
                    }
                }
            }
            cache[i] = ans;
            ans
        }

        let mut cache = vec![-1; s.len()];
        let s = s.as_str();
        decode(0, &s, &mut cache)
    }
}
