// 643. Maximum Average Subarray I
// -------------------------------

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut ans: f64 = std::f64::MIN;
        let n = nums.len();
        let mut window = 0;

        for i in 0..n {
            window += nums[i];

            if i >= (k - 1) as usize {
                let avg = window as f64 / k as f64;
                ans = ans.max(avg);
                window -= nums[i - (k - 1) as usize];
            }
        }

        ans
    }
}
