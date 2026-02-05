// 3379. Transformed Array
// -----------------------
impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = nums.clone();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                let k = (i + nums[i] as usize) % nums.len();
                ans[i] = nums[k];
            } else if nums[i] < 0 {
                let d = nums[i].abs() as usize;
                if d > i {
                    let k = (nums.len() - (d - i) % nums.len()) % nums.len();
                    ans[i] = nums[k];
                } else {
                    let k = i - d;
                    ans[i] = nums[k];
                }
            }
        }
        ans
    }
}
