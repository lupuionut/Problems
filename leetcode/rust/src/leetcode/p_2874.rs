// 2874. Maximum Value of an Ordered Triplet II
// --------------------------------------------

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut maximum_from = vec![-1; n];
        let mut curr_max = nums[n - 1];
        let mut best: i64 = -1;

        for i in (0..n).rev() {
            curr_max = curr_max.max(nums[i]);
            maximum_from[i] = curr_max;
        }

        let mut curr_max = 0;
        let mut curr_min = 1_000_001;

        for i in 0..n {
            if nums[i] > curr_max {
                curr_max = nums[i];
                curr_min = 1_000_001;
            } else {
                curr_min = curr_min.min(nums[i]);
            }

            if i >= 1 && i < n - 1 && curr_min < 1_000_001 {
                let possible = ((curr_max - curr_min) as i64) * (maximum_from[i + 1] as i64);
                if possible > 0 {
                    best = best.max(possible);
                }
            }
        }

        if best != -1 {
            best
        } else {
            0
        }
    }
}
