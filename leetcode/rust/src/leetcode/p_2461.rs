// 2461. Maximum Sum of Distinct Subarrays With Length K
// -----------------------------------------------------

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = vec![0; 100_001];
        let mut left = 0;
        let mut counter = 0;
        let mut running_sum = 0;
        let mut best = 0;

        for right in 0..nums.len() {
            running_sum += nums[right] as i64;
            count[nums[right] as usize] += 1;
            counter += count[nums[right] as usize];

            if (right - left + 1) > k as usize {
                counter -= count[nums[left] as usize];
                running_sum -= nums[left] as i64;
                count[nums[left] as usize] -= 1;
                left += 1;
            }
            if (right - left + 1) == k as usize {
                if counter == k {
                    best = best.max(running_sum);
                }
            }
        }

        best
    }
}
