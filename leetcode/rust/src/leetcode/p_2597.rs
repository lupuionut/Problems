// 2597. The Number of Beautiful Subsets
// -------------------------------------

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![0; 1001];

        fn count(i: usize, nums: &Vec<i32>, k: i32, counter: &mut Vec<i32>) -> i32 {
            if i == nums.len() {
                return 1;
            }
            let mut total = 0;
            total += count(i + 1, nums, k, counter);
            let k_minus = if (nums[i] - k) >= 0 {
                counter[(nums[i] - k) as usize]
            } else {
                0
            };

            let k_plus = if (nums[i] + k) <= 1000 {
                counter[(nums[i] + k) as usize]
            } else {
                0
            };

            if k_minus == 0 && k_plus == 0 {
                counter[(nums[i]) as usize] += 1;
                total += count(i + 1, nums, k, counter);
                counter[(nums[i]) as usize] -= 1;
            }
            total
        }
        count(0, &nums, k, &mut counter) - 1
    }
}
