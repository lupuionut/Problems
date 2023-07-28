// 486. Predict the Winner
// -----------------------

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut cache = vec![vec![-1; nums.len()]; nums.len()];

        fn score(nums: &Vec<i32>, left: usize, right: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
            if left == right {
                return nums[left];
            }
            if cache[left][right] != -1 {
                return cache[left][right];
            }
            let left_option = nums[left] - score(nums, left + 1, right, cache);
            let right_option = nums[right] - score(nums, left, right - 1, cache);

            cache[left][right] = left_option.max(right_option);
            cache[left][right]
        }
        return score(&nums, 0, nums.len() - 1, &mut cache) >= 0;
    }
}
