// 169. Majority Element
// ---------------------

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut e_val = 0;
        let mut e_freq = 0;

        nums.iter().for_each(|&n| {
            if n == e_val {
                e_freq += 1;
            } else {
                if e_freq == 0 {
                    e_val = n;
                    e_freq = 1;
                } else {
                    e_freq -= 1;
                }
            }
        });

        e_val
    }
}
