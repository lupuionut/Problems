// 1685. Sum of Absolute Differences in a Sorted Array
// ---------------------------------------------------

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        //      0------i------------n
        //      <---k--><----(n-k)-->
        // until i, all numbers will be less than nums[i] -> (k+1)*nums[i] - ps[i]
        // after i, all numbers will be larger than nums[i] -> ps[n-1] - ps[i] - (n-k-1)*nums[i]

        let n = nums.len();
        let mut ans = vec![0; n];
        let mut ps = vec![0; n];

        ps[0] = nums[0];
        for i in 1..n {
            ps[i] = ps[i - 1] + nums[i];
        }

        for i in 0..n {
            let left = i as i32 + 1;
            let right = (n - i) as i32 - 1;
            ans[i] = left * nums[i] - ps[i] + (ps[n - 1] - ps[i] - right * nums[i])
        }

        ans
    }
}
