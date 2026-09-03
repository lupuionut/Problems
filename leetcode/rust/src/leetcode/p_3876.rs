// 3876. Construct Uniform Parity Array II
// ---------------------------------------
impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut min_e = i32::MAX;
        let mut min_o = i32::MAX;

        for i in 0..nums1.len() {
            if nums1[i] % 2 == 0 {
                if min_e > nums1[i] {
                    min_e = nums1[i];
                }
            } else {
                if min_o > nums1[i] {
                    min_o = nums1[i];
                }
            }
        }

        let mut ans = true;
        // for even
        for i in 0..nums1.len() {
            if nums1[i] % 2 == 1 {
                if nums1[i] == min_o {
                    ans = false;
                }
            }
        }

        if ans == true {
            return true;
        }

        ans = true;
        for i in 0..nums1.len() {
            if nums1[i] % 2 == 0 {
                if min_o > nums1[i] {
                    ans = false;
                }
            }
        }

        ans
    }
}
