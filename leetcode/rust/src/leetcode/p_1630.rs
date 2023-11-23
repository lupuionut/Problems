// 1630. Arithmetic Subarrays
// --------------------------

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];

        for i in 0..l.len() {
            let start = l[i] as usize;
            let end = r[i] as usize;

            let mut intval = nums[start..=end].to_vec();
            intval.sort();
            let delta = intval[1] - intval[0];
            let mut val = true;

            for j in 1..intval.len() {
                if intval[j] - intval[j - 1] != delta {
                    val = false;
                    break;
                }
            }
            ans.push(val);
        }

        ans
    }
}
