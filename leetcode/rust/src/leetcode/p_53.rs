// 53. Maximum Subarray
// --------------------

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current_strike: Option<i32> = None;
        let mut best = nums[0];

        nums.iter().for_each(|&num| {
            if let Some(val) = current_strike {
                let new_curr_strike = num.max(val + num);
                current_strike = Some(new_curr_strike);
            } else {
                current_strike = Some(num);
            }
            best = best.max(current_strike.unwrap());
        });

        best
    }
}
