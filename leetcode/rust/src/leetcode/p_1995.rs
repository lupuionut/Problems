// 1995. Count Special Quadruplets
// -------------------------------

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let n = nums[i] + nums[j] + nums[k];
                    for l in k + 1..nums.len() {
                        if nums[l] == n {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}
