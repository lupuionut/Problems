// 2264. Largest 3-Same-Digit Number in String
// -------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let mut possible = HashMap::new();
        possible.insert("000", 0);
        possible.insert("111", 1);
        possible.insert("222", 2);
        possible.insert("333", 3);
        possible.insert("444", 4);
        possible.insert("555", 5);
        possible.insert("666", 6);
        possible.insert("777", 7);
        possible.insert("888", 8);
        possible.insert("999", 9);

        let mut ans = "";
        let mut best = -1;

        let num = num.as_str();
        for i in 0..num.len() - 2 {
            let key = &num[i..i + 3];
            if possible.contains_key(&key) {
                let val = *possible.get(&key).unwrap();
                if val > best {
                    best = val;
                    ans = key;
                }
            }
        }

        ans.to_string()
    }
}
