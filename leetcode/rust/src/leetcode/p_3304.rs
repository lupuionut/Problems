// 3304. Find the K-th Character in String Game I
// ----------------------------------------------

impl Solution {
    pub fn kth_character(k: i32) -> char {
        // a|b -> ab + b|c -> abbc + bc|cd -> abbcbccd + bccd|cdde -> abbcbccdbccdcdde + bccdcdde|cddedeef
        // total + (last one added unmodified + last one modified)
        fn next(c: u8) -> u8 {
            if c == b'z' {
                return b'a';
            }
            return c + 1;
        }

        fn transform(chars: &mut Vec<u8>, last: &mut Vec<u8>) {
            let mut t = last.clone();
            for i in 0..t.len() {
                last.push(next(t[i]));
            }
            chars.append(&mut last.clone())
        }

        let mut chars = vec![b'a', b'b'];
        let mut last = vec![b'b'];

        while chars.len() < k as usize {
            transform(&mut chars, &mut last);
        }

        char::from(chars[(k - 1) as usize])
    }
}
