// 2966. Divide Array Into Arrays With Max Difference
// --------------------------------------------------

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut possible = true;
        for i in 0..nums.len() / 3 {
            if (nums[i * 3] - nums[i * 3 + 2]).abs() > k {
                return vec![];
            }
        }
        nums.chunks(3).map(|c| c.to_vec()).collect()
    }
}
