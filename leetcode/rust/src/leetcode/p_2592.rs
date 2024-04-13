// 2592. Maximize Greatness of an Array
// ------------------------------------

impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut right = 0;
        let mut ans = 0;
        for left in 0..nums.len() {
            right += 1;
            while right < nums.len() && nums[left] == nums[right] {
                right += 1;
            }
            if right < nums.len() && nums[left] != nums[right] {
                ans += 1;
            }
        }

        ans
    }
}
