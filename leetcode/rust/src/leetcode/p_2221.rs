// 2221. Find Triangular Sum of an Array
// -------------------------------------

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut t = vec![];
            for i in 0..nums.len() - 1 {
                t.push((nums[i] + nums[i + 1]) % 10);
            }
            nums = t;
        }
        nums[0]
    }
}
