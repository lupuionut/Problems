// 1863. Sum of All Subset XOR Totals
// ----------------------------------

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        fn traverse(i: usize, acc: i32, nums: &Vec<i32>, ans: &mut i32) {
            if i == nums.len() {
                *ans += acc;
                return;
            }
            // include
            traverse(i + 1, acc ^ nums[i], nums, ans);

            // don't include
            traverse(i + 1, acc, nums, ans);
        }
        traverse(0, 0, &nums, &mut ans);
        ans
    }
}
