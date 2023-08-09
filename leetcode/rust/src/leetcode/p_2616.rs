// 2616. Minimize the Maximum Difference of Pairs
// ----------------------------------------------

impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        if p == 0 {
            return 0;
        }

        fn is_good(i: i32, p: i32, nums: &Vec<i32>) -> bool {
            let mut pairs = 0;
            let mut j = 0;
            while j < nums.len() - 1 {
                if (nums[j + 1] - nums[j]).abs() <= i {
                    j += 2;
                    pairs += 1;
                    if pairs == p {
                        return true;
                    }
                } else {
                    j += 1;
                }
            }
            false
        }

        let mut l = 0;
        let mut r = *nums.iter().max().unwrap() - *nums.iter().min().unwrap();
        let mut ans = r;

        while l <= r {
            let mid = l + (r - l) / 2;
            if is_good(mid, p, &nums) {
                ans = ans.min(mid);
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        ans
    }
}
