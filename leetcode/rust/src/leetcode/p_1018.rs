// 1018. Binary Prefix Divisible By 5
// ----------------------------------
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut curr = 0;
        let mut ans = vec![false; nums.len()];
        for i in 0..nums.len() {
            curr <<= 1;
            curr += nums[i];
            curr %= 1_000_000_000;
            if curr % 5 == 0 {
                ans[i] = true;
            }
        }
        ans
    }
}
