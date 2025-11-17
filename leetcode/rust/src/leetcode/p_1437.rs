// 1437. Check If All 1's Are at Least Length K Places Away
// --------------------------------------------------------
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last = None;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                if let Some(p) = last {
                    if i as i32 - p <= k {
                        return false;
                    }
                }
                last = Some(i as i32);
            }
        }
        true
    }
}
