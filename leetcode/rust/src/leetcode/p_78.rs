// 78. Subsets
// -----------

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn generate(i: usize, nums: &Vec<i32>, curr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                ans.push(curr.clone());
                return;
            }
            curr.push(nums[i]);
            generate(i + 1, nums, curr, ans);
            curr.pop();
            generate(i + 1, nums, curr, ans);
        }

        let mut curr = vec![];
        let mut ans = vec![];
        generate(0, &nums, &mut curr, &mut ans);
        ans
    }
}
