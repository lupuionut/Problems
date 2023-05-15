// 201. Bitwise AND of Numbers Range

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut ans = left;
        let mut l = left;
        let mut r = right;

        while l <= r {
            ans = ans & l;
            if ans == 0 {
                return 0;
            }
            if l == r {
                break;
            }
            l += 1;
        }
        ans
    }
}
