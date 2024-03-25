// 442. Find All Duplicates in an Array
// ------------------------------------

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ans = vec![];

        for i in 0..nums.len() {
            loop {
                let n = nums[i];
                let m = nums[n as usize - 1];

                if nums[n as usize - 1] == n {
                    break;
                }
                nums[i] = m;
                nums[n as usize - 1] = n;
            }
        }

        for i in 0..nums.len() {
            let n = nums[i];
            if i as i32 != n - 1 {
                ans.push(n);
            }
        }

        ans
    }
}
