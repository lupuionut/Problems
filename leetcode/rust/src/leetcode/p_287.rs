// 287. Find the Duplicate Number
// ------------------------------

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];
        let mut new_slow = nums[0];

        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }

        while slow != new_slow {
            slow = nums[slow as usize];
            new_slow = nums[new_slow as usize];
        }

        slow
    }
}
