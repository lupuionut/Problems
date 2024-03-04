// 948. Bag of Tokens
// ------------------

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        tokens.sort();

        if tokens.len() == 0 {
            return 0;
        }

        let mut left = 0;
        let mut right = tokens.len() - 1;
        let mut best = 0;
        let mut points = 0;
        let mut curr = power;

        while left <= right {
            if curr >= tokens[left] {
                points += 1;
                best = best.max(points);
                curr -= tokens[left];
                left += 1;
            } else if points >= 1 {
                curr += tokens[right];
                points -= 1;
                right -= 1;
            } else {
                break;
            }
        }

        best
    }
}
