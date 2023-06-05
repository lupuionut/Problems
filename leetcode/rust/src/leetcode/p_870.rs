// 870. Advantage Shuffle
// ----------------------

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut xs = nums1;
        let mut ys: Vec<(usize, i32)> = Vec::new();
        let mut ans = vec![-1; xs.len()];
        let mut remain: Vec<i32> = Vec::new();
        let mut i = 0;
        let mut j = 0;

        xs.sort();
        for (k, v) in nums2.iter().enumerate() {
            ys.push((k, *v));
        }
        ys.sort_by(|a, b| a.1.cmp(&b.1));

        while i < xs.len() && j < xs.len() {
            if xs[i] > ys[j].1 {
                ans[ys[j].0] = xs[i];
                i += 1;
                j += 1;
            } else {
                remain.push(xs[i]);
                i += 1;
            }
        }

        for i in 0..ans.len() {
            if ans[i] == -1 {
                let el = remain.pop().unwrap();
                ans[i] = el;
            }
        }

        ans
    }
}
