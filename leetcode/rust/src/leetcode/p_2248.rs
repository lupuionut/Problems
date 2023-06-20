// 2248. Intersection of Multiple Arrays
// -------------------------------------

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ns = vec![0; 1001];
        let mut ans: Vec<i32> = Vec::new();
        let n = nums.len();

        for i in 0..n {
            for j in 0..nums[i].len() {
                let key = nums[i][j] as usize;
                ns[key] += 1;
            }
        }

        for (k, v) in ns.iter().enumerate() {
            if *v == n {
                ans.push(k as i32);
            }
        }

        ans
    }
}
