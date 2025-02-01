// 3151. Special Array I
// ---------------------

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut w = nums.windows(2);
        let mut ans = true;
        while let Some(a) = w.next() {
            if (a[0] ^ a[1]) & 1 == 0 {
                return false;
            }
        }
        ans
    }
}
