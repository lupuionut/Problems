// 416. Partition Equal Subset Sum
// -------------------------------

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        let mut cache = vec![vec![None; sum as usize]; nums.len()];
        if sum % 2 == 1 {
            return false;
        }

        fn part(
            i: usize,
            nums: &Vec<i32>,
            sum: i32,
            total: i32,
            cache: &mut Vec<Vec<Option<bool>>>,
        ) -> bool {
            if i == nums.len() {
                if total == sum / 2 {
                    return true;
                }
                return false;
            }

            if cache[i][total as usize].is_some() {
                return cache[i][total as usize].unwrap();
            }

            cache[i][total as usize] = Some(
                part(i + 1, nums, sum, total + nums[i], cache)
                    || part(i + 1, nums, sum, total, cache),
            );
            cache[i][total as usize].unwrap()
        }

        part(0, &nums, sum, 0, &mut cache)
    }
}
