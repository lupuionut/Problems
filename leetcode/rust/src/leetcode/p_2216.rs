// 2216. Minimum Deletions to Make Array Beautiful
// -----------------------------------------------

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut min_del = 0;
        let mut cursor = 0;
        let mut last = nums[0];

        for i in 0..nums.len() {
            if cursor > 0 && cursor % 2 == 1 && last == nums[i] {
                min_del += 1;
                continue;
            }
            last = nums[i];
            cursor += 1;
        }

        if cursor % 2 == 1 {
            min_del += 1;
        }

        min_del
    }
}
