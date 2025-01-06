// 1769. Minimum Number of Operations to Move All Balls to Each Box
// ----------------------------------------------------------------

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut ans = vec![0; boxes.len()];
        let mut chars = boxes.chars().collect::<Vec<_>>();

        for i in 0..ans.len() {
            for j in 0..ans.len() {
                if i != j {
                    if chars[j] == '1' {
                        ans[i] += ((i - j) as i32).abs();
                    }
                }
            }
        }
        ans
    }
}
