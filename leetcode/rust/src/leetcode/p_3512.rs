// 3512. Minimum Operations to Make Array Sum Divisible by K
// ---------------------------------------------------------
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}
