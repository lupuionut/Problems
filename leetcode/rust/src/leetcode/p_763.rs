// 763. Partition Labels
// ---------------------

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut s = s.as_bytes();
        let mut last_appearance: Vec<usize> = vec![0; 26];
        let mut ans: Vec<i32> = vec![];

        for i in 0..s.len() {
            let k = s[i] - 97;
            last_appearance[k as usize] = i;
        }

        let k = (s[0] - 97) as usize;
        let mut limit = last_appearance[k];
        let mut delta = 0;

        for i in 0..s.len() {
            if limit == i {
                let position = (i + 1) as i32 - delta;
                delta += position;
                ans.push(position);
                if i + 1 < s.len() {
                    let k = (s[(i + 1)] - 97) as usize;
                    limit = last_appearance[k];
                }
            } else {
                let k = (s[i] - 97) as usize;
                let limitk = last_appearance[k];
                if limitk > limit {
                    limit = limitk;
                }
            }
        }

        ans
    }
}
