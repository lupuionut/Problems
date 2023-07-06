// 209. Minimum Size Subarray Sum
// ------------------------------

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut len = 100_001;
        let mut running_sum = 0;
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            running_sum += nums[right];
            while running_sum >= target {
                if running_sum - nums[left] < target {
                    break;
                }
                running_sum -= nums[left];
                left += 1;
            }
            if running_sum >= target {
                let delta = (right - left + 1) as i32;
                len = len.min(delta);
            }
            right += 1;
        }

        if len == 100_001 {
            0
        } else {
            len
        }
    }
}
