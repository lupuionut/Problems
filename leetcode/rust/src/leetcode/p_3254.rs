// 3254. Find the Power of K-Size Subarrays I
// ------------------------------------------

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let mut ans = vec![];
        let mut pairs = vec![1];
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                pairs.push(1);
            } else {
                pairs.push(0);
            }
        }

        let mut running_sum = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            let length = right - left + 1;
            if length > k as usize {
                left += 1;
                running_sum -= pairs[left];
            }

            running_sum += pairs[right];

            if (right - left + 1) == k as usize {
                if running_sum == k {
                    ans.push(nums[right]);
                } else {
                    ans.push(-1)
                }
            }
        }
        ans
    }
}
