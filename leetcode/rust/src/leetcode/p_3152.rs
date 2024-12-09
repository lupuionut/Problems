// 3152. Special Array II
// ----------------------

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut not_special = vec![];
        let mut ans = vec![true; queries.len()];

        for i in 0..nums.len() - 1 {
            if (nums[i] % 2 == 0 && nums[i + 1] % 2 == 0)
                || (nums[i] % 2 != 0 && nums[i + 1] % 2 != 0)
            {
                not_special.push(i as i32);
            }
        }

        for i in 0..queries.len() {
            let mut l = 0;
            let mut r = not_special.len();
            let start = queries[i][0] as i32;
            let end = queries[i][1] as i32;

            while l < r {
                let mid = (l + r) / 2;
                if not_special[mid] >= start && not_special[mid] < end {
                    ans[i] = false;
                    break;
                } else {
                    if not_special[mid] < start {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
            }
        }

        ans
    }
}
