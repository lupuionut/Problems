// 198. House Robber
// -----------------

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn recurse(curr: usize, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if curr >= nums.len() {
                return 0;
            }

            if cache[curr] != -1 {
                return cache[curr];
            }

            let take = nums[curr] + recurse(curr + 2, nums, cache);
            let skip = recurse(curr + 1, nums, cache);
            cache[curr] = take.max(skip);
            cache[curr]
        }

        let mut cache = vec![-1; nums.len()];
        recurse(0, &nums, &mut cache)
    }
}
