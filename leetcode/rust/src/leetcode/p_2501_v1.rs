// 2501. Longest Square Streak in an Array
// ---------------------------------------

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut max = *nums.iter().max().unwrap();
        let mut cache = vec![-1; (max + 1) as usize];

        fn calculate(n: i64, nums: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if cache[n as usize] != -1 {
                return cache[n as usize];
            }
            let mut total = 1;
            let mut target = n * n;
            let mut left = 0;
            let mut right = nums.len() - 1;
            let mut k = 0;

            while left <= right {
                let middle = (left + right) / 2;
                if nums[middle] as i64 == target {
                    k = middle;
                    break;
                }
                if (nums[middle] as i64) < target {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }

            if nums[k] as i64 == target {
                total += calculate(target, nums, cache);
            }

            cache[n as usize] = total;
            total
        }

        nums.iter().for_each(|&n| {
            calculate(n as i64, &nums, &mut cache);
        });

        let mut max = *cache.iter().max().unwrap();
        if max < 2 {
            return -1;
        }
        max
    }
}
