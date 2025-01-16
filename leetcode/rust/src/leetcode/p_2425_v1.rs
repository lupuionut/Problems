// 2425. Bitwise XOR of All Pairings
// ---------------------------------

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len() % 2;
        let m = nums2.len() % 2;
        let mut ans = 0;
        match (n, m) {
            (0, 1) => nums1.iter().for_each(|x| {
                ans ^= x;
            }),
            (1, 1) => {
                nums1.iter().for_each(|x| {
                    ans ^= x;
                });
                nums2.iter().for_each(|x| {
                    ans ^= x;
                })
            }
            (1, 0) => nums2.iter().for_each(|x| {
                ans ^= x;
            }),
            _ => {}
        }
        ans
    }
}
