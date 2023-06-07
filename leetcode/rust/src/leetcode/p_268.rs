// 268. Missing Number
// -------------------

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let n = nums.len() as i32;
        let expected = (n * (n + 1)) / 2;
        expected - total
    }
}
