// 1415. The k-th Lexicographical String of All Happy Strings of Length n
// ----------------------------------------------------------------------

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut m = 1;
        let mut bytes = vec![b'a', b'b', b'c'];
        let mut comb: Vec<Vec<u8>> = vec![vec![b'a'], vec![b'b'], vec![b'c']];
        while m < n {
            let words = comb.clone();
            comb.clear();
            for word in words {
                for b in &bytes {
                    if b != &word[(m - 1) as usize] {
                        let mut w = word.clone();
                        w.push(*b);
                        comb.push(w);
                    }
                }
            }
            m += 1;
        }

        if comb.len() < k as usize {
            return "".to_string();
        }

        let ans = match String::from_utf8(comb[(k - 1) as usize].clone()) {
            Ok(s) => s.to_string(),
            Err(_) => "".to_string(),
        };
        ans
    }
}
