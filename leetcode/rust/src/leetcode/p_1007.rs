// 1007. Minimum Domino Rotations For Equal Row
// --------------------------------------------

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut freq = [0; 7];
        for i in 0..tops.len() {
            let a = tops[i] as usize;
            let b = bottoms[i] as usize;
            freq[a] += 1;
            freq[b] += 1;
        }

        let req = tops.len() as i32;
        let mut s = -1;
        for i in 0..=6 {
            if freq[i] >= req {
                if s == -1 {
                    s = i as i32;
                }
            }
        }

        let mut top_best = 0;
        let mut bottom_best = 0;
        for i in 0..tops.len() {
            if tops[i] != s && bottoms[i] != s {
                return -1;
            }
            if tops[i] != s {
                top_best += 1;
            }
            if bottoms[i] != s {
                bottom_best += 1;
            }
        }

        top_best.min(bottom_best)
    }
}
