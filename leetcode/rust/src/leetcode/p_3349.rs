// 3349. Adjacent Increasing Subarrays Detection I
// -----------------------------------------------
impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut i = 0;

        while i <= nums.len() - 2 * k {
            let mut inc = true;
            for j in i..i + k {
                if j >= (i + 1) {
                    if nums[j - 1] >= nums[j] {
                        inc = false;
                    }
                }
            }

            if inc == false {
                i += 1;
                continue;
            }

            inc = true;
            for j in i + k..i + 2 * k {
                if j >= i + k + 1 {
                    if nums[j - 1] >= nums[j] {
                        inc = false;
                    }
                }
            }
            if inc == true {
                return true;
            }
            i += 1;
        }
        false
    }
}
