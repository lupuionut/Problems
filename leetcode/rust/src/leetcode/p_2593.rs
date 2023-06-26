// 2593. Find Score of an Array After Marking All Elements
// -------------------------------------------------------

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut pairs: Vec<(usize, i32)> = vec![(0, 0); nums.len()];
        let mut used: Vec<bool> = vec![false; nums.len()];
        let mut score = 0;

        for (k, v) in nums.iter().enumerate() {
            pairs[k] = (k, *v);
        }

        pairs.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        for pair in &pairs {
            if used[pair.0] == false {
                used[pair.0] = true;
                if pair.0 + 1 < pairs.len() {
                    used[pair.0 + 1] = true;
                }
                if pair.0 >= 1 {
                    used[pair.0 - 1] = true;
                }
                score += pair.1 as i64;
            }
        }

        score
    }
}
