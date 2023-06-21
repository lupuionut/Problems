// 2448. Minimum Cost to Make Array Equal
// -------------------------------------

impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        fn cost_for(val: i32, nums: &Vec<i32>, cost: &Vec<i32>) -> i64 {
            let mut total = 0;
            for i in 0..nums.len() {
                total += (nums[i] - val).abs() as i64 * cost[i] as i64;
            }
            total
        }

        let mut left = *nums.iter().min().unwrap();
        let mut right = *nums.iter().max().unwrap();

        while left <= right {
            let middle = (left + right) / 2;
            if cost_for(middle, &nums, &cost) < cost_for(middle + 1, &nums, &cost) {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        }

        cost_for(left, &nums, &cost).min(cost_for(right, &nums, &cost))
    }
}
