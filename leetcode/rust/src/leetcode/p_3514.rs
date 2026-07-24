// 3514. Number of Unique XOR Triplets II
// --------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let mut ones = HashSet::new();
        let mut twos = HashSet::new();
        let mut threes = HashSet::new();

        for i in 0..nums.len() {
            ones.insert(nums[i]);
            for val in ones.iter() {
                twos.insert(nums[i] ^ val);
            }
            for val in twos.iter() {
                threes.insert(nums[i] ^ val);
            }
        }

        threes.len() as i32
    }
}
