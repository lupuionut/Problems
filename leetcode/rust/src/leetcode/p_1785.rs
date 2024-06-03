// 1785. Minimum Elements to Add to Form a Given Sum
// -------------------------------------------------

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let mut sum = 0;
        nums.iter().for_each(|&n| sum += n as i64);
        let need = (goal as i64 - sum).abs();

        if need % limit as i64 != 0 {
            (need / limit as i64) as i32 + 1
        } else {
            (need / limit as i64) as i32
        }
    }
}
