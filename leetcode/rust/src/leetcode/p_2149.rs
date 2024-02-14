// 2149. Rearrange Array Elements by Sign
// --------------------------------------

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positives = vec![];
        let mut negatives = vec![];
        nums.iter().for_each(|&n| {
            if n >= 0 {
                positives.push(n);
            } else {
                negatives.push(n);
            }
        });

        let mut ans = vec![];
        let mut pairs = positives.iter().zip(negatives.iter());
        for (&p, &n) in pairs {
            ans.push(p);
            ans.push(n);
        }

        ans
    }
}
