// 821. Shortest Distance to a Character
// -------------------------------------

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![0; s.len()];
        let mut s = s.as_bytes();
        let c = c as u8;
        let mut last = -1;
        let mut temp = vec![(0, 0); s.len()];

        for i in 0..s.len() {
            if s[i] == c {
                last = i as i32;
            } else {
                if last != -1 {
                    temp[i].0 = (last - i as i32).abs();
                }
            }
        }

        last = -1;
        for i in (0..s.len()).rev() {
            if s[i] == c {
                last = i as i32;
            } else {
                if last != -1 {
                    temp[i].1 = (last - i as i32).abs();
                }
            }
        }

        for i in 0..temp.len() {
            match temp[i] {
                (f, 0) => {
                    ans[i] = f;
                }
                (0, s) => {
                    ans[i] = s;
                }
                (f, s) => {
                    ans[i] = f.min(s);
                }
            }
        }

        ans
    }
}
