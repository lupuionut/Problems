// 2425. Bitwise XOR of All Pairings
// ---------------------------------

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();

        let mut nums3: Vec<i32> = vec![];
        nums1.iter().for_each(|&n| {
            if n2 & 1 == 1 {
                nums3.push(n);
            }
        });

        nums2.iter().for_each(|&n| {
            if n1 & 1 == 1 {
                nums3.push(n);
            }
        });

        let mut ans = 0;
        if nums3.len() > 0 {
            ans = nums3.into_iter().reduce(|acc, n| acc ^ n).unwrap()
        }
        ans
    }
}
