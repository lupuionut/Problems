// 2460. Apply Operations to an Array
// ----------------------------------

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        for i in 0..ans.len() - 1 {
            if ans[i] == ans[i + 1] {
                ans[i] *= 2;
                ans[i + 1] = 0;
            }
        }

        ans = ans.into_iter().filter(|&x| x != 0).collect();

        while ans.len() < nums.len() {
            ans.push(0);
        }

        ans
    }
}
