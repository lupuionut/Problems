// 139. Word Break
// ---------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cache {
    Uninit,
    Init(bool),
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut cache = vec![Cache::Uninit; s.len() + 1];

        fn can_break(i: usize, s: &str, dict: &Vec<String>, cache: &mut Vec<Cache>) -> bool {
            if i == s.len() {
                return true;
            }

            if cache[i] != Cache::Uninit {
                match cache[i] {
                    Cache::Init(true) => return true,
                    Cache::Init(false) => return false,
                    _ => return false,
                }
            }

            let n = s.len() - i;
            for w in dict {
                let wl = w.len();
                if wl > n {
                    continue;
                }
                if &s[i..i + wl] == w.as_str() {
                    cache[i] = Cache::Init(true);
                    if can_break(i + wl, s, dict, cache) {
                        return true;
                    }
                }
            }
            cache[i] = Cache::Init(false);
            false
        }
        can_break(0, s.as_str(), &word_dict, &mut cache)
    }
}
