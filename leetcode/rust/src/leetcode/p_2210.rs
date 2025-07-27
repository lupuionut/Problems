// 2210. Count Hills and Valleys in an Array
// -----------------------------------------

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut f = -1;
        let mut s = -1;
        let mut ans = 0;
        for i in 0..nums.len() {
            if f == -1 {
                f = nums[i];
                continue;
            }
            if s == -1 {
                if nums[i] != f {
                    s = nums[i];
                }
                continue;
            }
            if nums[i] == s {
                continue;
            }
            if (f < s && nums[i] < s) || (f > s && nums[i] > s) {
                ans += 1;
            }
            f = s;
            s = nums[i];
        }
        ans
    }
}
