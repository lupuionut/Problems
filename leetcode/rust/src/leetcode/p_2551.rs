// 2551. Put Marbles in Bags
// -------------------------

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            return 0;
        }
        let mut cuts = vec![];
        let mut iter = weights.windows(2);
        while let Some(xs) = iter.next() {
            cuts.push((xs[0] + xs[1]) as i64);
        }
        cuts.sort();
        let n = cuts.len();
        let mx: i64 = cuts[n - (k - 1) as usize..].iter().sum();
        let mn: i64 = cuts[0..(k - 1) as usize].iter().sum();

        mx - mn
    }
}
