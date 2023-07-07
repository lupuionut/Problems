// 2399. Check Distances Between Same Letters
// ------------------------------------------

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut appearance: Vec<i32> = vec![-1; 26];
        let mut s = s.as_bytes();

        for i in 0..s.len() {
            let k = (s[i] - 97) as usize;
            if appearance[k] != -1 {
                if (i - appearance[k] as usize - 1) as i32 != distance[k] {
                    return false;
                }
            }
            appearance[k] = i as i32;
        }

        true
    }
}
