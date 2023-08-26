// 646. Maximum Length of Pair Chain
// ---------------------------------

impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs;
        let mut ans = 1;

        pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut last = &pairs[0];

        pairs.iter().for_each(|p| {
            if last[1] < p[0] {
                ans += 1;
                last = p;
            }
        });

        ans
    }
}
