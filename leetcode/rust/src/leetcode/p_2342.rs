// 2342. Max Sum of a Pair With Equal Sum of Digits
// ------------------------------------------------

use std::collections::HashMap;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        fn digit_sum(mut n: i32) -> i32 {
            let mut ans = 0;
            while n > 0 {
                ans += n % 10;
                n /= 10;
            }
            ans
        }

        let mut ans = -1;
        let mut sums: HashMap<i32, [i32; 2]> = HashMap::new();
        for i in 0..nums.len() {
            let s = digit_sum(nums[i]);
            sums.entry(s)
                .and_modify(|vals| {
                    if vals[0] == 0 {
                        vals[0] = nums[i];
                    } else if vals[1] == 0 {
                        vals[1] = nums[i];
                    } else {
                        if vals[0] > vals[1] && nums[i] > vals[1] {
                            vals[1] = nums[i];
                        } else if nums[i] > vals[0] {
                            vals[0] = nums[i];
                        }
                    }
                    ans = ans.max(vals[0] + vals[1]);
                })
                .or_insert([0, nums[i]]);
        }

        ans
    }
}
