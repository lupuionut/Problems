// 724. Find Pivot Index
// ---------------------

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let n = nums.len();

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        for i in 1..nums.len() {
            let left = prefix_sum[i - 1];
            let right = prefix_sum[n] - prefix_sum[i];
            if left == right {
                return (i - 1) as i32;
            }
        }
        if prefix_sum[n - 1] == 0 {
            return (n - 1) as i32;
        }
        -1
    }
}
