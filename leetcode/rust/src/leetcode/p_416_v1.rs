// 416. Partition Equal Subset Sum
// -------------------------------

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let mut cache = vec![vec![None; nums.len()]; (sum / 2 + 1) as usize];
        fn traverse(
            i: usize,
            cache: &mut Vec<Vec<Option<bool>>>,
            nums: &Vec<i32>,
            target: i32,
        ) -> bool {
            if i == nums.len() {
                if target == 0 {
                    return true;
                }
                return false;
            }
            if target == 0 {
                return true;
            }
            if let Some(v) = cache[target as usize][i] {
                return v;
            }
            let mut ans = false;
            if nums[i] <= target {
                ans = ans || traverse(i + 1, cache, nums, target - nums[i]);
            }
            ans = ans || traverse(i + 1, cache, nums, target);
            cache[target as usize][i] = Some(ans);
            ans
        }

        traverse(0, &mut cache, &nums, sum / 2)
    }
}
