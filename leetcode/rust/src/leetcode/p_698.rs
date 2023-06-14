// 698. Partition to K Equal Sum Subsets
// -------------------------------------

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let total: i32 = nums.iter().sum();
        let target = total / k;
        let mut visited = 0;
        if total % k != 0 {
            return false;
        }

        fn backtrack(
            nums: &Vec<i32>,
            i: usize,
            current_sum: i32,
            target: i32,
            visited: &mut i32,
            k: i32,
        ) -> bool {
            if k == 0 {
                return true;
            }
            if current_sum == target {
                return backtrack(nums, 0, 0, target, visited, k - 1);
            }
            for j in i..nums.len() {
                if *visited & (1 << j) == 0 && current_sum + nums[j] <= target {
                    let temp = *visited;
                    *visited = *visited | (1 << j);
                    if backtrack(nums, j + 1, current_sum + nums[j], target, visited, k) == true {
                        return true;
                    }
                    *visited = temp;
                }
            }
            return false;
        }

        backtrack(&nums, 0, 0, target, &mut visited, k)
    }
}
