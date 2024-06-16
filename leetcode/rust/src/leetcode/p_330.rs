// 330. Patching Array
// -------------------

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut can_reach = 0i64;
        let mut ans = 0;

        for i in 0..nums.len() {
            while can_reach < n as i64 && can_reach + 1 < nums[i] as i64 {
                can_reach += (can_reach + 1);
                ans += 1;
            }
            can_reach += nums[i] as i64;
        }

        while can_reach < n as i64 {
            can_reach += can_reach + 1;
            ans += 1;
        }

        ans
    }
}
