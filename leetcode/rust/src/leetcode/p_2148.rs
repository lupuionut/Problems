// 2148. Count Elements With Strictly Smaller and Greater Elements
// ---------------------------------------------------------------

impl Solution {
    pub fn count_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let n = nums.len() - 1;
        nums.iter().for_each(|&x| {
            if x > nums[0] && x < nums[n] {
                ans += 1;
            }
        });
        ans
    }
}
