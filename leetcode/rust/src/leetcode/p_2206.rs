// 2206. Divide Array Into Equal Pairs
// -----------------------------------

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>) -> bool {
        nums.sort();
        let mut i = 0;
        while i < nums.len() {
            if nums[i] != nums[i + 1] {
                return false;
            }
            i += 2;
        }
        true
    }
}
