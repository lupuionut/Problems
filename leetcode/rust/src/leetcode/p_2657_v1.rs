// 2657. Find the Prefix Common Array of Two Arrays
// ------------------------------------------------

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![(0, 0); 51];
        let mut ans = vec![0; a.len()];

        for i in 0..a.len() {
            let k0 = a[i] as usize;
            let k1 = b[i] as usize;
            freq[k0].0 += 1;
            freq[k1].1 += 1;
            let mut p = 0;
            for j in 0..51 {
                if freq[j].0 == freq[j].1 && freq[j].0 != 0 {
                    p += 1;
                }
            }
            ans[i] = p;
        }
        ans
    }
}
