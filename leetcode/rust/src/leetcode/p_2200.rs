// 2200. Find All K-Distant Indices in an Array
// --------------------------------------------

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut curr = -1;
        let mut ans = vec![];

        for i in 0..nums.len() {
            if nums[i] == key {
                let d = i.min(k as usize);
                let l = (i - d) as i32;
                let r = (i + k as usize).min(nums.len() - 1) as i32;
                let s = (curr + 1).max(l);
                for j in s..=r {
                    ans.push(j);
                }
                curr = r;
            }
        }

        ans
    }
}
