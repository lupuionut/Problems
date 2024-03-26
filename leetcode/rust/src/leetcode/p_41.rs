// 41. First Missing Positive
// --------------------------

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            loop {
                let n = nums[i];
                if n <= 0 || n > nums.len() as i32 {
                    break;
                }
                let m = nums[n as usize - 1];

                if nums[n as usize - 1] == n {
                    break;
                }
                nums[i] = m;
                nums[n as usize - 1] = n;
            }
        }
        for i in 0..nums.len() {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }

        return nums.len() as i32 + 1;
    }
}
