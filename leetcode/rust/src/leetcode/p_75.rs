// 75. Sort Colors
// ---------------

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut end_0 = -1;
        let mut end_1 = -1;
        let mut end_2 = -1;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                end_0 += 1;
                nums[end_0 as usize] = 0;

                if end_1 != -1 {
                    end_1 += 1;
                    nums[end_1 as usize] = 1;
                }
                if end_2 != -1 {
                    end_2 += 1;
                    nums[end_2 as usize] = 2;
                }
                continue;
            }

            if nums[i] == 1 {
                if end_1 == -1 {
                    end_1 = end_0 + 1;
                } else {
                    end_1 += 1;
                }
                nums[end_1 as usize] = 1;
                if end_2 != -1 {
                    end_2 += 1;
                    nums[end_2 as usize] = 2;
                }
                continue;
            }

            if nums[i] == 2 {
                if end_2 == -1 {
                    if end_1 == -1 {
                        end_2 = end_0 + 1;
                    } else {
                        end_2 = end_1 + 1;
                    }
                } else {
                    end_2 += 1;
                }
                nums[end_2 as usize] = 2;
            }
        }
    }
}
