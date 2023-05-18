// 2374. Node With Highest Edge Score

impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut scores = vec![0; edges.len()];
        let mut max = (0, 0);

        for (from, to) in edges.iter().enumerate() {
            scores[*to as usize] += from;
        }

        for (node, score) in scores.iter().enumerate() {
            if *score > max.1 {
                max.1 = *score;
                max.0 = node;
            }
        }

        max.0 as i32
    }
}
