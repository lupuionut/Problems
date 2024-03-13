// 2475. Number of Unequal Triplets in Array
// -----------------------------------------

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        fn dp(i: usize, used: &mut Vec<i32>, nums: &Vec<i32>) -> i32 {
            if i == nums.len() {
                return 0;
            }
            let mut ans = 0;
            let mut j = i + 1;
            while j < nums.len() {
                let mut good = false;
                if used.len() == 1 {
                    if nums[j] != used[0] {
                        used.push(nums[j]);
                        ans += dp(j, used, nums);
                        used.pop();
                    }
                } else {
                    if nums[j] != used[0] && nums[j] != used[1] {
                        ans += 1;
                    }
                }

                j += 1;
            }
            ans
        }

        let mut ans = 0;
        let mut used = vec![];
        for i in 0..nums.len() {
            used.push(nums[i]);
            ans += dp(i, &mut used, &nums);
            used.pop();
        }
        ans
    }
}
