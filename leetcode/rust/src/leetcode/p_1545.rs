// 1545. Find Kth Bit in Nth Binary String
// ---------------------------------------

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let mut cache = vec![None; (n + 2) as usize];
        cache[0] = Some("0".to_string());
        cache[1] = Some("0".to_string());
        let s = Self::create(n, &mut cache);
        s.chars().nth((k - 1) as usize).unwrap()
    }

    pub fn create(n: i32, cache: &mut Vec<Option<String>>) -> String {
        if let Some(s) = &cache[n as usize] {
            return s.to_string();
        }

        let previous = Self::create(n - 1, cache);
        let reverted = previous
            .clone()
            .chars()
            .map(|c| if c == '0' { '1' } else { '0' })
            .rev()
            .collect::<String>();
        let mut curr = vec![];
        curr.push(previous);
        curr.push("1".to_string());
        curr.push(reverted);
        let t = curr.concat();
        cache[n as usize] = Some(t.clone());
        t
    }
}
