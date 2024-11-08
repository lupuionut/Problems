// 1829. Maximum XOR for Each Query
// --------------------------------

impl Solution {
    pub fn get_maximum_xor(mut nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut curr = nums.clone().into_iter().reduce(|acc, e| acc ^ e).unwrap();
        let mut ans = vec![];
        let max = (1 << maximum_bit) - 1;

        while nums.len() > 0 {
            ans.push(max ^ curr);
            curr = (curr ^ nums.pop().unwrap());
        }
        ans
    }
}
