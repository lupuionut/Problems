// 763. Partition Labels
// ---------------------

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];
        let mut idx = 0;
        let mut ans = vec![];

        for &c in s.as_bytes() {
            let k = c - 97;
            last[k as usize] = idx;
            idx += 1;
        }

        let mut left = 0;
        let mut right = 0;
        let mut max = 0;

        for &c in s.as_bytes() {
            let k = (c - 97) as usize;
            max = max.max(last[k]);
            if right == max {
                ans.push(right - left + 1);
                left = right + 1;
            }
            right += 1;
        }

        ans
    }
}
