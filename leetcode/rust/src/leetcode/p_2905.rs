// 2905. Find Indices With Index and Value Difference II
// -----------------------------------------------------

impl Solution {
    pub fn find_indices(
        mut nums: Vec<i32>,
        index_difference: i32,
        value_difference: i32,
    ) -> Vec<i32> {
        let mut min_idx = 0;
        let mut max_idx = 0;
        let delta = index_difference as usize;

        for i in delta..nums.len() {
            if nums[i - delta] > nums[max_idx] {
                max_idx = i - delta;
            }
            if nums[i - delta] < nums[min_idx] {
                min_idx = i - delta;
            }
            if (nums[i] - nums[min_idx]).abs() >= value_difference {
                return vec![i as i32, min_idx as i32];
            }
            if (nums[i] - nums[max_idx]).abs() >= value_difference {
                return vec![i as i32, max_idx as i32];
            }
        }

        vec![-1, -1]
    }
}
