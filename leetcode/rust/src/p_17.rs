// 17. Letter Combinations of a Phone Number
// -----------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut symbols: HashMap<u8, &[u8]> = HashMap::new();
        symbols.insert(b'2', &[b'a', b'b', b'c']);
        symbols.insert(b'3', &[b'd', b'e', b'f']);
        symbols.insert(b'4', &[b'g', b'h', b'i']);
        symbols.insert(b'5', &[b'j', b'k', b'l']);
        symbols.insert(b'6', &[b'm', b'n', b'o']);
        symbols.insert(b'7', &[b'p', b'q', b'r', b's']);
        symbols.insert(b'8', &[b't', b'u', b'v']);
        symbols.insert(b'9', &[b'w', b'x', b'y', b'z']);
        let mut ans = vec![];

        fn combine(
            s: &[u8],
            i: usize,
            current_comb: &mut Vec<u8>,
            ans: &mut Vec<String>,
            symbols: &HashMap<u8, &[u8]>,
        ) {
            if s.len() == 0 {
                return;
            }
            if current_comb.len() == s.len() {
                let st = String::from_utf8(current_comb.to_vec()).unwrap();
                ans.push(st);
                return;
            }
            let values = symbols.get(&s[i]).unwrap();
            for j in 0..values.len() {
                current_comb.push(values[j]);
                combine(s, i + 1, current_comb, ans, symbols);
                current_comb.pop();
            }
        }

        combine(digits.as_bytes(), 0, &mut vec![], &mut ans, &symbols);
        ans
    }
}
