// 905. Sort Array By Parity
// -------------------------

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut l = 0;
        let mut r = nums.len() - 1;

        nums.iter().for_each(|&x| {
            if x & 1 == 1 {
                ans[r] = x;
                r -= 1;
            } else {
                ans[l] = x;
                l += 1;
            }
        });

        ans
    }
}
