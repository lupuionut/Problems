// 2551. Put Marbles in Bags
// -------------------------

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, mut k: i32) -> i64 {
        let mut cuts = vec![(0, 0); weights.len() - 1];
        for i in 1..weights.len() {
            cuts[i - 1] = (weights[i - 1] + weights[i], i - 1);
        }

        cuts.sort();
        let last = cuts.len();
        let mut maximum = weights[0] as i64 + weights[last] as i64;
        let mut minimum = weights[0] as i64 + weights[last] as i64;
        let mut l = 0;
        let mut r = last - 1;
        k -= 1;

        if k == 0 {
            return 0;
        }

        while k > 0 {
            let (val, idx) = cuts[l];
            minimum += val as i64;
            let (val, idx) = cuts[r];
            maximum += val as i64;

            k -= 1;
            l += 1;
            r -= 1;
        }
        maximum - minimum
    }
}
