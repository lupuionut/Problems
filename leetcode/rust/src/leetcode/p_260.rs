// 260. Single Number III
// ----------------------

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xored = 0;
        let mut a = 0;
        let mut b = 0;

        nums.iter().for_each(|&n| {
            xored ^= n;
        });

        let mask = xored & -xored;

        nums.iter().for_each(|&n| {
            if n & mask == mask {
                a ^= n;
            } else {
                b ^= n;
            }
        });
        vec![a, b]
    }
}
