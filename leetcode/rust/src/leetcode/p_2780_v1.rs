// 2780. Minimum Index of a Valid Split
// ------------------------------------

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut dominant = -1;
        let mut dominant_counter = 0;

        for &num in &nums {
            if num == dominant {
                dominant_counter += 1;
            } else {
                dominant_counter -= 1;
                if dominant_counter < 0 {
                    dominant = num;
                    dominant_counter = 1;
                }
            }
        }

        dominant_counter = 0;
        for &num in &nums {
            if num == dominant {
                dominant_counter += 1;
            }
        }

        let mut seen = 0;
        let mut ans = -1;
        let n = nums.len();

        for i in 0..n {
            if nums[i] == dominant {
                seen += 1;
            }
            if seen > ((i + 1) as i32 / 2) && (dominant_counter - seen > (n - i - 1) as i32 / 2) {
                ans = i as i32;
                break;
            }
        }

        ans
    }
}
