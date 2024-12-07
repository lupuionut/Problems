// 1760. Minimum Limit of Balls in a Bag
// -------------------------------------

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = 1_000_000_000;
        let mut ans = i32::MAX;

        fn is_good(nums: &Vec<i32>, n: i32, max_operations: i32) -> bool {
            let mut used = 0;
            for i in 0..nums.len() {
                if nums[i] > n {
                    if used == max_operations {
                        return false;
                    } else {
                        let mut to_use = nums[i] / n;
                        if nums[i] % n == 0 {
                            to_use -= 1;
                        }
                        if (to_use + used) > max_operations {
                            return false;
                        }
                        used += to_use;
                    }
                }
            }
            true
        }

        while left <= right {
            let middle = (left + right) / 2;
            if is_good(&nums, middle, max_operations) {
                ans = ans.min(middle);
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        ans
    }
}
