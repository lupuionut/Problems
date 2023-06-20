// 2090. K Radius Subarray Averages
// --------------------------------

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut running_sum: Vec<i64> = vec![0; nums.len() + 1];
        let mut ans = vec![0; nums.len()];

        let mut left = 0 - k;
        let mut right = k + 1;

        for i in 0..nums.len() {
            running_sum[i + 1] = running_sum[i] + nums[i] as i64;
        }

        for i in 0..nums.len() {
            if left < 0 || right > nums.len() as i32 {
                ans[i] = -1;
            } else {
                let frame_len = 2 * k + 1;
                let frame_sum =
                    running_sum[right as usize] - running_sum[(right - frame_len) as usize];
                ans[i] = (frame_sum / frame_len as i64) as i32;
            }
            left += 1;
            right += 1;
        }
        ans
    }
}
