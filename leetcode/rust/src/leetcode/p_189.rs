// 189. Rotate Array
// -----------------

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let mut k = (k as usize) % n;
        let mut temp: Vec<i32> = vec![];

        if k == 0 {
            return;
        }

        for i in (n - k)..n {
            temp.push(nums[i]);
        }
        for i in 0..(n - k) {
            temp.push(nums[i]);
        }
        *nums = temp;
    }
}
