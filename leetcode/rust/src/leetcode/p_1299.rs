// 1299. Replace Elements with Greatest Element on Right Side
// ----------------------------------------------------------

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut greatest = -1;
        let mut ans = vec![-1; arr.len()];

        for i in (0..arr.len()).rev() {
            ans[i] = greatest;
            greatest = greatest.max(arr[i]);
        }

        ans
    }
}
