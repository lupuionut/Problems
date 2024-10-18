// 2044. Count Number of Maximum Bitwise-OR Subsets
// ------------------------------------------------
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn recurse(i: usize, nums: &Vec<i32>, curr: i32, best: &mut (i32, i32)) {
            if i == nums.len() {
                return;
            }
            // include curr number
            let x = curr | nums[i];
            if x == best.0 {
                (*best).1 += 1;
            }
            if x > best.0 {
                *best = (x, 1);
            }
            recurse(i + 1, &nums, curr, best);
            recurse(i + 1, &nums, x, best);
        }
        let mut best = (0, 0);
        recurse(0, &nums, 0, &mut best);
        best.1
    }
}
