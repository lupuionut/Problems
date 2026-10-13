// 3727. Maximum Alternating Sum of Squares
// ----------------------------------------
impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_by(|a,b| {(b*b).cmp(&(a*a))});
        let mut ans = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let a = (nums[l] * nums[l]) as i64;
            let b = (nums[r] * nums[r]) as i64;
            ans += (a - b);
            l += 1;
            r -= 1;
        }

        if l == r {
            ans += (nums[l] * nums[l]) as i64;
        }
        ans
    }
}
