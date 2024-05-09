// 506. Relative Ranks
// -------------------

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut scores = score.clone();
        scores.sort_by(|a, b| b.cmp(&a));
        let mut ans = vec![];

        for i in 0..score.len() {
            let pos = Self::find_index(&scores, score[i]);
            ans.push(Self::index_to_str(pos));
        }

        ans
    }

    fn find_index(scores: &Vec<i32>, n: i32) -> usize {
        let mut left = 0;
        let mut right = scores.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if scores[mid] == n {
                return mid;
            } else if scores[mid] < n {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn index_to_str(i: usize) -> String {
        match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => (i + 1).to_string(),
        }
    }
}
