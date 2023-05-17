// 1863. Sum of All Subset XOR Totals

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut subsets: Vec<i32> = vec![];
        subsets.push(0);

        for i in 0..nums.len() {
            for j in subsets.clone() {
                subsets.push(nums[i] ^ j as i32);
            }
        }
        subsets.into_iter().reduce(|acc, e| acc + e).unwrap()
    }
}
