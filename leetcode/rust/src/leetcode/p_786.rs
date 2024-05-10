// 786. K-th Smallest Prime Fraction
// ---------------------------------

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut fractions = vec![];
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                fractions.push((arr[i], arr[j]));
            }
        }
        fractions.sort_by(|a, b| (a.0 * b.1).cmp(&(a.1 * b.0)));
        let ans = fractions[(k - 1) as usize];
        vec![ans.0, ans.1]
    }
}
