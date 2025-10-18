// 3397. Maximum Number of Distinct Elements After Operations
// ----------------------------------------------------------

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut last = nums[0] - k + 1;
        let mut ans = 1;
        for i in 1..nums.len() {
            let mut f = nums[i] - k;
            f = f.max(last);
            if f <= nums[i] {
                ans += 1;
                last = f + 1;
            } else {
                if f <= nums[i] + k {
                    f = f.min(nums[i] + k);
                    last = f + 1;
                    ans += 1;
                }
            }
        }
        ans
    }
}
