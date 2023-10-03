// 1512. Number of Good Pairs
// --------------------------
// number of pairs for each x, is (n-1)*n/2
// for ex. if we have 5 1's, we will have a total of 4+3+2+1

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut pairs = vec![0; 101];
        nums.iter().for_each(|&x| pairs[x as usize] += 1);
        pairs.iter().map(|x| (x - 1) * x / 2).sum::<i32>()
    }
}
