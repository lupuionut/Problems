// 645. Set Mismatch
// -----------------

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; nums.len() + 1];
        let mut ans = vec![0; 2];
        nums.iter().for_each(|&n| {
            let k = n as usize;
            freq[k] += 1;
        });

        freq.iter().enumerate().for_each(|(k, &v)| {
            if v == 2 {
                ans[0] = k as i32;
            }
            if v == 0 {
                ans[1] = k as i32;
            }
        });
        ans
    }
}
