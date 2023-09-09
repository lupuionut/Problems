// 377. Combination Sum IV
// -----------------------

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache = vec![-1; (target + 1) as usize];

        fn combine(t: i32, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if t < 0 {
                return 0;
            }
            if t == 0 {
                return 1;
            }
            if cache[t as usize] != -1 {
                return cache[t as usize];
            }
            let mut total = 0;
            for n in nums.iter() {
                total += combine(t - n, nums, cache);
            }
            cache[t as usize] = total;
            total
        }

        combine(target, &nums, &mut cache)
    }
}
